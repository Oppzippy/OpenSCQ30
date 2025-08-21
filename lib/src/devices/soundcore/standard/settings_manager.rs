use std::{
    collections::{HashMap, hash_map},
    marker::PhantomData,
};

use async_trait::async_trait;
use thiserror::Error;

use crate::{
    api::settings::{self, CategoryId, Setting, SettingId, Value, ValueError},
    storage,
};

#[derive(Copy, Clone)]
struct SettingHandlerKey(usize);

pub struct SettingsManager<T> {
    categories: Vec<CategoryId>,
    categories_to_settings: HashMap<CategoryId, Vec<SettingId>>,
    settings_to_handlers: HashMap<SettingId, SettingHandlerKey>,
    handlers: Vec<Box<dyn SettingHandler<T> + Send + Sync>>,
}

impl<T> Default for SettingsManager<T> {
    fn default() -> Self {
        Self {
            categories: Vec::new(),
            categories_to_settings: HashMap::new(),
            settings_to_handlers: HashMap::new(),
            handlers: Vec::new(),
        }
    }
}

impl<StateType> SettingsManager<StateType> {
    pub fn add_handler<T: SettingHandler<StateType> + 'static + Send + Sync>(
        &mut self,
        category: CategoryId,
        handler: T,
    ) {
        let settings = handler.settings();
        let handler_key = SettingHandlerKey(self.handlers.len());
        self.handlers.push(Box::new(handler));

        match self.categories_to_settings.entry(category) {
            hash_map::Entry::Occupied(mut occupied_entry) => {
                occupied_entry.get_mut().extend_from_slice(&settings);
            }
            hash_map::Entry::Vacant(vacant_entry) => {
                self.categories.push(*vacant_entry.key());
                vacant_entry.insert(settings.to_vec());
            }
        }
        settings.into_iter().for_each(|setting| {
            self.settings_to_handlers.insert(setting, handler_key);
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
        let handler = self.handler(setting_id)?;
        handler.get(state, setting_id)
    }

    pub async fn set(
        &self,
        state: &mut StateType,
        setting_id: &SettingId,
        value: Value,
    ) -> Option<SettingHandlerResult<()>> {
        let handler = self.handler(setting_id)?;
        Some(handler.set(state, setting_id, value).await)
    }

    fn handler(
        &self,
        setting_id: &SettingId,
    ) -> Option<&(dyn SettingHandler<StateType> + Send + Sync)> {
        let handler_key = self.settings_to_handlers.get(setting_id)?;
        match self.handlers.get(handler_key.0) {
            Some(handler) => Some(handler.as_ref()),
            None => {
                tracing::error!(
                    "{setting_id} has a handler key assigned, but no handler with the key exists"
                );
                None
            }
        }
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
    #[error("setting does not exist")]
    DoesNotExist,
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

/// Should only be used by OptionalSettingHandler
pub enum AsRefMut<'a, T> {
    Ref(&'a T),
    Mut(&'a mut T),
}
impl<'a, T> AsRef<T> for AsRefMut<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            AsRefMut::Ref(r) => r,
            AsRefMut::Mut(m) => m,
        }
    }
}
impl<'a, T> AsMut<T> for AsRefMut<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        match self {
            AsRefMut::Ref(_r) => unreachable!(),
            AsRefMut::Mut(m) => m,
        }
    }
}

pub struct OptionalSettingHandler<Inner, OptionTRef, T>
where
    Inner: for<'a> SettingHandler<AsRefMut<'a, T>> + Send + Sync,
    OptionTRef: AsRef<Option<T>> + AsMut<Option<T>> + Send + Sync,
    T: Send + Sync,
{
    inner: Inner,
    _a: PhantomData<OptionTRef>,
    _b: PhantomData<T>,
}

impl<Inner, OptionTRef, T> OptionalSettingHandler<Inner, OptionTRef, T>
where
    Inner: for<'a> SettingHandler<AsRefMut<'a, T>> + Send + Sync,
    OptionTRef: AsRef<Option<T>> + AsMut<Option<T>> + Send + Sync,
    T: Send + Sync,
{
    pub fn new(inner: Inner) -> Self {
        Self {
            inner,
            _a: PhantomData,
            _b: PhantomData,
        }
    }
}

#[async_trait]
impl<Inner, OptionTRef, T> SettingHandler<OptionTRef>
    for OptionalSettingHandler<Inner, OptionTRef, T>
where
    Inner: for<'a> SettingHandler<AsRefMut<'a, T>> + Send + Sync,
    OptionTRef: AsRef<Option<T>> + AsMut<Option<T>> + Send + Sync,
    T: Send + Sync,
{
    fn settings(&self) -> Vec<SettingId> {
        self.inner.settings()
    }

    fn get(&self, maybe_state: &OptionTRef, setting_id: &SettingId) -> Option<Setting> {
        if let Some(state) = maybe_state.as_ref() {
            self.inner.get(&AsRefMut::Ref(state), setting_id)
        } else {
            None
        }
    }

    async fn set(
        &self,
        maybe_state: &mut OptionTRef,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        if let Some(state) = maybe_state.as_mut() {
            self.inner
                .set(&mut AsRefMut::Mut(state), setting_id, value)
                .await
        } else {
            Err(SettingHandlerError::DoesNotExist)
        }
    }
}
