use std::{
    collections::{HashMap, hash_map},
    sync::Arc,
};

use async_trait::async_trait;
use thiserror::Error;

use crate::{
    api::settings::{self, CategoryId, Setting, SettingId, Value, ValueError},
    storage,
};

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

    pub async fn set(
        &self,
        state: &mut StateType,
        setting_id: &SettingId,
        value: Value,
    ) -> Option<SettingHandlerResult<()>> {
        let handler = self.settings_to_handlers.get(setting_id)?;
        Some(handler.set(state, setting_id, value).await)
    }
}

#[async_trait]
pub trait SettingHandler<T> {
    fn settings(&self) -> Vec<SettingId>;
    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting>;
    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()>;
}

pub type SettingHandlerResult<T> = Result<T, SettingHandlerError>;

#[derive(Debug, Error)]
pub enum SettingHandlerError {
    #[error("value")]
    ValueError(#[from] ValueError),
    #[error("storage")]
    StorageError(#[from] storage::Error),
    #[error("setting is read only")]
    ReadOnly,
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl SettingHandlerError {
    #[track_caller]
    pub fn into_settings_error(self, setting_id: SettingId) -> settings::Error {
        settings::Error {
            setting_id,
            source: Box::new(self),
        }
    }
}
