use std::{
    collections::{HashMap, HashSet, hash_map},
    sync::Arc,
};

use async_trait::async_trait;

use crate::api::settings::{CategoryId, Setting, SettingId, Value};

pub struct SettingsManager<T> {
    categories: Vec<CategoryId>,
    categories_to_settings: HashMap<CategoryId, Vec<SettingId>>,
    settings_to_handlers: HashMap<SettingId, Arc<dyn SettingHandler<T> + Send + Sync>>,
}

impl<T> Default for SettingsManager<T> {
    fn default() -> Self {
        Self {
            categories: Vec::new(),
            categories_to_settings: HashMap::new(),
            settings_to_handlers: HashMap::new(),
        }
    }
}

impl<StateType> SettingsManager<StateType> {
    pub fn add_handler<T: SettingHandler<StateType> + 'static + Send + Sync>(
        &mut self,
        category: CategoryId,
        handler: T,
    ) {
        let handler = Arc::new(handler);
        let settings = handler.settings();

        match self.categories_to_settings.entry(category) {
            hash_map::Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().extend_from_slice(&settings);
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                self.categories.push(vacant_entry.key().clone());
                vacant_entry.insert(settings.to_vec());
            }
        }
        settings.into_iter().for_each(|setting| {
            self.settings_to_handlers
                .insert(setting, handler.to_owned());
        });
    }

    pub fn categories(&self) -> &[CategoryId] {
        &self.categories
    }

    pub fn category(&self, category: &CategoryId) -> Vec<SettingId> {
        self.categories_to_settings
            .get(category)
            .cloned()
            .unwrap_or_default()
    }

    pub fn get(&self, state: &StateType, setting_id: &SettingId) -> Option<Setting> {
        let handler = self.settings_to_handlers.get(setting_id)?;
        handler.get(state, setting_id)
    }

    pub fn get_many(
        &self,
        state: &StateType,
        desired_setting_ids: &HashSet<SettingId>,
    ) -> HashMap<SettingId, Value> {
        self.settings_to_handlers
            .iter()
            .filter_map(|(id, handler)| {
                if desired_setting_ids.contains(id) {
                    Some((id.clone(), handler.get(state, id)?.into()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_all(&self, state: &StateType) -> HashMap<SettingId, Value> {
        self.settings_to_handlers
            .iter()
            .map(|(setting_id, handler)| (setting_id.clone(), handler.get(state, setting_id)))
            .filter_map(|(setting_id, maybe_value)| {
                maybe_value.map(|value| (setting_id, value.into()))
            })
            .collect()
    }

    pub async fn set(
        &self,
        state: &mut StateType,
        setting_id: &SettingId,
        value: Value,
    ) -> Option<crate::Result<()>> {
        let handler = self.settings_to_handlers.get(setting_id)?;
        Some(handler.set(state, setting_id, value).await)
    }
}

#[async_trait]
pub trait SettingHandler<T> {
    fn settings(&self) -> Vec<SettingId>;
    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting>;
    async fn set(&self, state: &mut T, setting_id: &SettingId, value: Value) -> crate::Result<()>;
}
