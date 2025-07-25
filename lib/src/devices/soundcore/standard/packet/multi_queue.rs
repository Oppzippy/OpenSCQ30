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
    pub fn pop(&self, key: &K, value: V) -> bool {
        let mut queues = self.queues.lock().expect(LOCK_HELD_ERROR);
        if let Some(queue) = queues.get_mut(key) {
            if let Some(front) = queue.pop_front() {
                *front.value.lock().expect("mutex should not be tainted") = Some(value);
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
            // No need to close the semaphore since we were given ownership of the only outside reference to it
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
    pub async fn wait_for_end(&self) {
        self.current
            .semaphore
            .acquire()
            .await
            .expect_err(PERMIT_ACQUIRED_ERROR);
    }

    pub async fn wait_for_value(self) -> T {
        self.wait_for_end().await;
        self.current
            .value
            .lock()
            .expect("mutex should not be tainted")
            .take()
            .expect("in order to cancel, ownership must be given away, but we still have ownership, so we must have a value")
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::RangeInclusive, time::Duration};

    use super::*;

    #[tokio::test(start_paused = true)]
    async fn test_independent_queues() {
        const RANGE: RangeInclusive<i8> = 1..=5;
        let queues: MultiQueue<i8, i8> = MultiQueue::new();
        let mut queue1_handles = RANGE.map(|_| queues.add(1)).collect::<VecDeque<_>>();
        let mut queue2_handles = RANGE.map(|_| queues.add(2)).collect::<VecDeque<_>>();

        for i in RANGE {
            queues.pop(&1, i);
            queues.pop(&2, -i);
        }
        for i in RANGE {
            let handle = queue1_handles.pop_front().unwrap();
            assert_eq!(
                tokio::time::timeout(Duration::from_millis(1), handle.wait_for_value()).await,
                Ok(i)
            );
        }
        for i in RANGE {
            let handle = queue2_handles.pop_front().unwrap();
            assert_eq!(
                tokio::time::timeout(Duration::from_millis(1), handle.wait_for_value()).await,
                Ok(-i)
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
        queues.pop(&0, 1);
        queues.pop(&0, 3);
        assert_eq!(
            tokio::time::timeout(Duration::from_millis(1), first.wait_for_value()).await,
            Ok(1)
        );
        assert_eq!(
            tokio::time::timeout(Duration::from_millis(1), third.wait_for_value()).await,
            Ok(3)
        );
    }
}
