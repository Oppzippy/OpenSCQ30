use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    sync::Arc,
};

use tokio::sync::Semaphore;

const PERMIT_ACQUIRED_ERROR: &str = "expected closed semaphore, permit should never be acquired";
const LOCK_HELD_ERROR: &str =
    "lock should not already be held by current thread. was it held across an await?";

#[derive(Debug)]
struct SemaphoreWithValue<T> {
    semaphore: Semaphore,
    value: std::sync::Mutex<Option<T>>,
}

/// For each key, maintains a queue of pending tasks. The idea is each key can have its own sequence of
/// tasks that must occur sequentially.
#[derive(Default, Debug)]
pub struct MultiQueue<K: Hash + Eq, V> {
    queues: std::sync::Mutex<HashMap<K, VecDeque<Arc<SemaphoreWithValue<V>>>>>,
}

impl<K: Hash + Eq, V> MultiQueue<K, V> {
    pub fn new() -> Self {
        Self {
            queues: Default::default(),
        }
    }

    /// Adds a new task to the end of the queue for the specified key and returns a handle for the
    /// newly added task.
    pub fn add(&self, key: K) -> MultiQueueHandle<V> {
        let mut queues = self.queues.lock().expect(LOCK_HELD_ERROR);
        let queue = queues.entry(key).or_default();
        let preceeding = queue.back().cloned();

        let current = Arc::new(SemaphoreWithValue {
            semaphore: Semaphore::new(0),
            value: std::sync::Mutex::new(None),
        });
        queue.push_back(current.clone());

        MultiQueueHandle {
            preceeding,
            current,
        }
    }

    /// When the action being performed in the task at the front of the queue is done, this will assign
    /// a value as a result of the task and pop it from the queue.
    pub fn pop(&self, key: &K, value: Option<V>) -> bool {
        let mut queues = self.queues.lock().expect(LOCK_HELD_ERROR);
        if let Some(queue) = queues.get_mut(key) {
            if let Some(front) = queue.pop_front() {
                *front.value.lock().expect("mutex should not be tainted") = value;
                front.semaphore.close();
                return true;
            }
        }
        false
    }

    /// Removes a task from a queue without returning a result
    pub fn cancel(&self, key: &K, handle: MultiQueueHandle<V>) {
        let mut queues = self.queues.lock().expect(LOCK_HELD_ERROR);
        if let Some(queue) = queues.get_mut(key)
            && let Some(index) = queue
                .iter()
                .position(|entry| Arc::ptr_eq(entry, &handle.current))
        {
            queue.remove(index);
        }
    }
}

pub struct MultiQueueHandle<T> {
    preceeding: Option<Arc<SemaphoreWithValue<T>>>,
    current: Arc<SemaphoreWithValue<T>>,
}

impl<T> MultiQueueHandle<T> {
    /// Waits for this handle to be at the front of the queue
    pub async fn wait_for_start(&self) {
        if let Some(preceeding) = &self.preceeding {
            preceeding
                .semaphore
                .acquire()
                .await
                .expect_err(PERMIT_ACQUIRED_ERROR);
        }
    }

    /// Waits for this handle to be popped from the queue and returns its result
    pub async fn wait_for_end(&self) -> Option<T> {
        self.current
            .semaphore
            .acquire()
            .await
            .expect_err(PERMIT_ACQUIRED_ERROR);
        self.current
            .value
            .lock()
            .expect("mutex should not be tainted")
            .take()
    }
}

#[cfg(test)]
mod tests {
    use std::{array, ops::RangeInclusive, time::Duration};

    use tokio::select;

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn test_independent_queues() {
        const ITERATIONS: i8 = 5;
        let queues: MultiQueue<i8, i8> = MultiQueue::new();
        let queue1_handles: [_; ITERATIONS as usize] = array::from_fn(|_| queues.add(1));
        let queue2_handles: [_; ITERATIONS as usize] = array::from_fn(|_| queues.add(2));

        const RANGE: RangeInclusive<i8> = 1i8..=ITERATIONS;
        for i in RANGE {
            queues.pop(&1, Some(i));
            queues.pop(&2, Some(-i));
        }
        for i in RANGE {
            assert_eq!(queue1_handles[i as usize - 1].wait_for_end().await, Some(i));
        }
        for i in RANGE {
            assert_eq!(
                queue2_handles[i as usize - 1].wait_for_end().await,
                Some(-i)
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn test_cancel() {
        let queues = MultiQueue::<i8, i8>::new();
        let first = queues.add(0);
        let second = queues.add(0);
        let third = queues.add(0);
        queues.cancel(&0, second);
        queues.pop(&0, Some(1));
        queues.pop(&0, Some(3));
        assert_eq!(first.wait_for_end().await, Some(1));
        select! {
            result = third.wait_for_end() => assert_eq!(result, Some(3)),
            _ = tokio::time::sleep(Duration::from_millis(1)) => panic!("timed out"),
        }
    }
}
