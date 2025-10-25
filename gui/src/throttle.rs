use std::{collections::HashMap, mem, sync::Arc, time::Duration};

use cosmic::Task;
use openscq30_lib::{
    device::{self, OpenSCQ30Device},
    settings::{Setting, SettingId, Value},
};

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Trigger,
    Error(Arc<device::Error>),
}

pub enum Action {
    Task(Task<Message>),
    Error(Arc<device::Error>),
    None,
}

pub struct Throttle {
    device: Arc<dyn OpenSCQ30Device + Send + Sync>,
    queue: HashMap<SettingId, Value>,
    is_pending: bool,
}

impl Throttle {
    pub fn new(device: Arc<dyn OpenSCQ30Device + Send + Sync>) -> Self {
        Self {
            device,
            queue: HashMap::new(),
            is_pending: false,
        }
    }

    pub fn set_setting(&mut self, setting_id: SettingId, value: Value) -> Option<Task<Message>> {
        self.queue.insert(setting_id, value);
        if self.is_pending {
            None
        } else {
            self.is_pending = true;
            Some(Task::future(async {
                tokio::time::sleep(Duration::from_millis(200)).await;
                Message::Trigger
            }))
        }
    }

    pub fn setting(&self, setting_id: &SettingId) -> Option<Setting> {
        let mut setting = self.device.setting(setting_id)?;

        let queued_value = self.queue.get(setting_id).cloned();

        // TODO implement the rest
        match &mut setting {
            Setting::Toggle { value: _ } => (),
            Setting::I32Range { value, .. } => {
                if let Some(v) = queued_value {
                    *value = v.try_as_i32().unwrap();
                }
            }
            Setting::Select { value: _, .. } => (),
            Setting::OptionalSelect { value: _, .. } => (),
            Setting::ModifiableSelect { value: _, .. } => (),
            Setting::MultiSelect { values: _, .. } => (),
            Setting::Equalizer { value, .. } => {
                if let Some(v) = queued_value {
                    *value = v.try_into_i16_vec().unwrap();
                }
            }
            Setting::Information { .. } => (),
            Setting::ImportString { .. } => (),
            Setting::Action => (),
        }

        Some(setting)
    }

    pub fn trigger(&mut self) -> Task<Message> {
        self.is_pending = false;
        let queue = mem::take(&mut self.queue);
        let device = self.device.clone();
        Task::future(async move {
            if let Err(err) = device.set_setting_values(queue.into_iter().collect()).await {
                Message::Error(Arc::new(err))
            } else {
                Message::None
            }
        })
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::None => Action::None,
            Message::Trigger => Action::Task(self.trigger()),
            Message::Error(err) => Action::Error(err),
        }
    }
}
