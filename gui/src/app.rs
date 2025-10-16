use std::{collections::VecDeque, ops::Deref, path::PathBuf, sync::Arc};

use cosmic::{
    Application, ApplicationExt, Apply, Task,
    app::{Core, context_drawer::ContextDrawer},
    iced::{Length, alignment},
    widget::{self, nav_bar},
};
use macaddr::MacAddr6;
use openscq30_i18n::Translate;
use openscq30_lib::{OpenSCQ30Session, device::OpenSCQ30Device, storage::PairedDevice};
use tokio::{select, sync::Semaphore};

use crate::{
    add_device::{self, AddDeviceModel},
    device_selection::{self, DeviceSelectionModel},
    device_settings, fl,
    utils::coalesce_result,
};

pub struct AppModel {
    core: Core,
    screen: Screen,
    dialog_page: Option<DialogPage>,
    session: Arc<OpenSCQ30Session>,
    warnings: VecDeque<String>,
    config_dir: PathBuf,
    about: widget::about::About,
    context_drawer_screen: Option<ContextDrawerScreen>,
}
pub struct AppFlags {
    pub config_dir: PathBuf,
}

enum ContextDrawerScreen {
    About,
}

#[derive(Debug, Clone)]
pub enum Message {
    DeviceSelectionScreen(device_selection::Message),
    AddDeviceScreen(add_device::Message),
    DeviceSettingsScreen(device_settings::Message),
    CloseDialog,
    RemovePairedDevice(MacAddr6),
    BackToDeviceSelection,
    ActivateConnectToDeviceScreen(DebugOpenSCQ30Device),
    CloseDialogAndRefreshPairedDevices,
    ActivateDeviceSelectionScreen,
    Warning(String),
    CloseWarning,
    ShowAbout,
    OpenUrl(String),
    CloseContextDrawer,
    ConnectToDeviceFailed(String),
    CancelConnectToDevice,
    None,
}

impl From<device_selection::Message> for Message {
    fn from(message: device_selection::Message) -> Self {
        Self::DeviceSelectionScreen(message)
    }
}
impl From<add_device::Message> for Message {
    fn from(message: add_device::Message) -> Self {
        Self::AddDeviceScreen(message)
    }
}
impl From<device_settings::Message> for Message {
    fn from(message: device_settings::Message) -> Self {
        Self::DeviceSettingsScreen(message)
    }
}
#[derive(Clone)]
pub struct DebugOpenSCQ30Device(pub Arc<dyn OpenSCQ30Device + Send + Sync>);
impl std::fmt::Debug for DebugOpenSCQ30Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenSCQ30Device").finish()
    }
}
impl Deref for DebugOpenSCQ30Device {
    type Target = Arc<dyn OpenSCQ30Device + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum DialogPage {
    RemoveDevice(PairedDevice),
}

#[allow(clippy::large_enum_variant)]
enum Screen {
    DeviceSelection(device_selection::DeviceSelectionModel),
    AddDevice(add_device::AddDeviceModel),
    Connecting {
        canceled: Arc<Semaphore>,
        name: String,
    },
    DeviceSettings(device_settings::DeviceSettingsModel),
}

// This is a macro so that the file/line number of the tracing message matches the caller
#[macro_export]
macro_rules! handle_soft_error {
    () => {
        |err| {
            let err = ::anyhow::Error::from(err);
            ::tracing::warn!("soft_error: {err:?}");
            Message::Warning($crate::fl!("error-with-message", err = format!("{err:#}")))
        }
    };
}

impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = AppFlags;
    type Message = Message;

    const APP_ID: &'static str = "com.oppzippy.OpenSCQ30";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let about = widget::about::About::default()
            .name(fl!("openscq30"))
            .icon(crate::icons::openscq30())
            .version(env!("CARGO_PKG_VERSION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .license(env!("CARGO_PKG_LICENSE"))
            .links([(env!("CARGO_PKG_REPOSITORY"), env!("CARGO_PKG_REPOSITORY"))]);

        let session = Arc::new(
            futures::executor::block_on(OpenSCQ30Session::new(
                flags.config_dir.join("database.sqlite"),
            ))
            .expect("database is required to run"),
        );
        let (model, task) = DeviceSelectionModel::new(session.clone());
        let mut app = Self {
            core,
            screen: Screen::DeviceSelection(model),
            dialog_page: None,
            session,
            warnings: VecDeque::with_capacity(5),
            config_dir: flags.config_dir,
            about,
            context_drawer_screen: None,
        };
        let command = app.update_title();
        (
            app,
            cosmic::Task::batch([
                command,
                task.map(Message::DeviceSelectionScreen).map(Into::into),
            ]),
        )
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        match &self.screen {
            Screen::DeviceSettings(model) => model.nav_model(),
            _ => None,
        }
    }

    fn on_nav_select(&mut self, id: nav_bar::Id) -> cosmic::app::Task<Self::Message> {
        match &mut self.screen {
            Screen::DeviceSettings(model) => model
                .on_nav_select(id)
                .map(Message::DeviceSettingsScreen)
                .map(Into::into),
            _ => unreachable!("no nav bar is shown, so selecting an item is impossible"),
        }
    }

    fn header_start(&self) -> Vec<cosmic::Element<'_, Self::Message>> {
        match self.screen {
            Screen::DeviceSelection(_) => vec![
                // shown on device selection screen not because it's relevant to device selection, but because it is
                // the default screen
                widget::button::icon(crate::icons::help_about_symbolic())
                    .on_press(Message::ShowAbout)
                    .into(),
            ],
            _ => vec![
                widget::button::icon(crate::icons::go_previous_symbolic())
                    .on_press(Message::BackToDeviceSelection)
                    .into(),
            ],
        }
    }

    fn view(&self) -> cosmic::Element<'_, Self::Message> {
        widget::column()
            .push_maybe(
                self.warnings
                    .front()
                    .map(|message| widget::warning(message).on_close(Message::CloseWarning)),
            )
            .push(match &self.screen {
                Screen::DeviceSelection(device_selection_model) => cosmic::Element::from(
                    device_selection_model
                        .view()
                        .map(Message::DeviceSelectionScreen),
                ),
                Screen::AddDevice(add_device_model) => {
                    add_device_model.view().map(Message::AddDeviceScreen)
                }
                Screen::Connecting { canceled: _, name } => self.view_cancel(name),
                Screen::DeviceSettings(device_settings_model) => device_settings_model
                    .view()
                    .map(Message::DeviceSettingsScreen),
            })
            .into()
    }

    fn dialog(&self) -> Option<cosmic::Element<'_, Self::Message>> {
        let dialog = match &self.screen {
            Screen::DeviceSelection(_device_selection_model) => None,
            Screen::AddDevice(_add_device_model) => None,
            Screen::Connecting { .. } => None,
            Screen::DeviceSettings(device_settings_model) => device_settings_model
                .dialog()
                .map(|e| e.map(Message::DeviceSettingsScreen)),
        };
        if dialog.is_some() {
            return dialog;
        }
        let dialog_page = self.dialog_page.as_ref()?;
        Some(match dialog_page {
            DialogPage::RemoveDevice(device) => widget::dialog()
                .title(fl!("prompt-remove-device-title"))
                .body(fl!("prompt-remove-device", name = device.model.translate()))
                .icon(widget::icon(crate::icons::dialog_warning_symbolic()))
                .primary_action(
                    widget::button::destructive(fl!("remove"))
                        .on_press(Message::RemovePairedDevice(device.mac_address)),
                )
                .secondary_action(
                    widget::button::text(fl!("cancel")).on_press(Message::CloseDialog),
                )
                .into(),
        })
    }

    fn context_drawer(&self) -> Option<ContextDrawer<'_, Self::Message>> {
        if let Some(context_drawer_screen) = &self.context_drawer_screen {
            match context_drawer_screen {
                ContextDrawerScreen::About => Some(cosmic::app::context_drawer::about(
                    &self.about,
                    |url| Message::OpenUrl(url.to_owned()),
                    Message::CloseContextDrawer,
                )),
            }
        } else {
            match &self.screen {
                Screen::DeviceSelection(_device_selection_model) => None,
                Screen::AddDevice(_add_device_model) => None,
                Screen::Connecting { .. } => None,
                Screen::DeviceSettings(device_settings_model) => device_settings_model
                    .context_drawer()
                    .map(|drawer| drawer.map(Message::DeviceSettingsScreen)),
            }
        }
    }

    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::None => (),
            Message::DeviceSelectionScreen(message) => {
                if let Screen::DeviceSelection(ref mut screen) = self.screen {
                    match screen.update(message) {
                        device_selection::Action::ConnectToDevice(paired_device) => {
                            let session = self.session.clone();
                            let canceled = Arc::new(Semaphore::new(0));
                            self.screen = Screen::Connecting {
                                canceled: canceled.clone(),
                                name: paired_device.model.translate(),
                            };
                            return Task::future(async move {
                                let connect_result = select! {
                                    connect_result = session.connect(paired_device.mac_address) => connect_result,
                                    _ = canceled.acquire() => return Ok(Message::None.into()),
                                };

                                match connect_result {
                                    Ok(device) => {
                                        Ok(Message::ActivateConnectToDeviceScreen(DebugOpenSCQ30Device(
                                            device,
                                        ))
                                        .into())
                                    },
                                    Err(err) => {
                                        let err = anyhow::Error::from(err);
                                        tracing::warn!("soft_error: {err:?}");
                                        Ok(Message::ConnectToDeviceFailed(fl!(
                                                "error-with-message",
                                                err = format!("{err:#}")
                                        )).into())
                                    },
                                }
                            })
                            .map(coalesce_result);
                        }
                        device_selection::Action::RemoveDevice(device) => {
                            self.dialog_page = Some(DialogPage::RemoveDevice(device));
                        }
                        device_selection::Action::AddDevice => {
                            self.screen =
                                Screen::AddDevice(AddDeviceModel::new(self.session.clone()));
                        }
                        device_selection::Action::None => (),
                        device_selection::Action::Warning(message) => {
                            return Task::done(Message::Warning(message).into());
                        }
                    }
                }
            }
            Message::AddDeviceScreen(message) => {
                if let Screen::AddDevice(ref mut screen) = self.screen {
                    match screen.update(message) {
                        add_device::Action::None => (),
                        add_device::Action::Task(task) => {
                            return task.map(Message::AddDeviceScreen).map(Into::into);
                        }
                        add_device::Action::AddDevice(paired_device) => {
                            let database = self.session.clone();
                            return Task::future(async move {
                                database
                                    .pair(paired_device)
                                    .await
                                    .map_err(handle_soft_error!())?;
                                Ok(Message::ActivateDeviceSelectionScreen.into())
                            })
                            .map(coalesce_result);
                        }
                    }
                }
            }
            Message::ActivateDeviceSelectionScreen => {
                let (model, task) = DeviceSelectionModel::new(self.session.clone());
                self.screen = Screen::DeviceSelection(model);
                return task.map(Message::DeviceSelectionScreen).map(Into::into);
            }
            Message::DeviceSettingsScreen(message) => {
                if let Screen::DeviceSettings(ref mut screen) = self.screen {
                    match screen.update(message) {
                        device_settings::Action::Task(task) => {
                            return task.map(Message::DeviceSettingsScreen).map(Into::into);
                        }
                        device_settings::Action::None => (),
                        device_settings::Action::Warning(message) => {
                            return Task::done(Message::Warning(message).into());
                        }
                        device_settings::Action::FocusTextInput(id) => {
                            return widget::text_input::focus(id);
                        }
                        device_settings::Action::Disconnect => {
                            return Task::done(Message::ActivateDeviceSelectionScreen.into());
                        }
                    }
                }
            }
            Message::CloseDialog => self.dialog_page = None,
            Message::RemovePairedDevice(mac_address) => {
                let database = self.session.clone();
                return Task::future(async move {
                    database
                        .unpair(mac_address)
                        .await
                        .map_err(handle_soft_error!())?;
                    Ok(Message::CloseDialogAndRefreshPairedDevices.into())
                })
                .map(coalesce_result);
            }
            Message::CloseDialogAndRefreshPairedDevices => {
                if let Screen::DeviceSelection(ref mut _screen) = self.screen {
                    self.dialog_page = None;
                    return device_selection::DeviceSelectionModel::refresh_paired_devices(
                        self.session.clone(),
                    )
                    .map(Message::from)
                    .map(Into::into);
                }
            }
            Message::BackToDeviceSelection => {
                let (model, task) = DeviceSelectionModel::new(self.session.clone());
                self.screen = Screen::DeviceSelection(model);
                return task.map(Message::DeviceSelectionScreen).map(Into::into);
            }
            Message::CancelConnectToDevice => {
                if let Screen::Connecting { canceled, .. } = &self.screen {
                    canceled.close();
                    return Task::done(Message::ActivateDeviceSelectionScreen.into());
                }
            }
            Message::ConnectToDeviceFailed(message) => {
                return Task::batch([
                    Task::done(Message::ActivateDeviceSelectionScreen.into()),
                    Task::done(Message::Warning(message).into()),
                ]);
            }
            Message::ActivateConnectToDeviceScreen(device) => {
                let (model, task) = device_settings::DeviceSettingsModel::new(
                    device,
                    self.session.quick_preset_handler(),
                    self.config_dir.to_owned(),
                );
                self.screen = Screen::DeviceSettings(model);
                return task.map(Message::DeviceSettingsScreen).map(Into::into);
            }
            Message::Warning(message) => {
                // cap max number of warnings, since it's bad UX to have to close a million of them if something goes wrong and spams them
                if self.warnings.capacity() == self.warnings.len() {
                    self.warnings.pop_front();
                }
                self.warnings.push_back(message);
            }
            Message::CloseWarning => {
                self.warnings.pop_front();
            }
            Message::CloseContextDrawer => self.context_drawer_screen = None,
            Message::ShowAbout => self.context_drawer_screen = Some(ContextDrawerScreen::About),
            Message::OpenUrl(url) => {
                if let Err(err) = open::that_detached(&url) {
                    tracing::error!("error opening url {url}: {err:?}");
                }
            }
        }
        Task::none()
    }
}

impl AppModel {
    pub fn update_title(&mut self) -> cosmic::app::Task<Message> {
        if let Some(id) = self.core.main_window_id() {
            self.set_header_title(fl!("openscq30"));
            self.set_window_title(fl!("openscq30"), id)
        } else {
            Task::none()
        }
    }

    fn view_cancel(&self, device_name: &str) -> cosmic::Element<'_, Message> {
        widget::column()
            .spacing(10)
            .align_x(alignment::Horizontal::Center)
            .push(widget::text::title2(fl!(
                "connecting-to",
                name = device_name
            )))
            .push(
                widget::button::destructive(fl!("cancel")).on_press(Message::CancelConnectToDevice),
            )
            .apply(widget::container)
            .center(Length::Fill)
            .into()
    }
}
