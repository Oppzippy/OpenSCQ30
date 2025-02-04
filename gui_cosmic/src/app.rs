use std::{ops::Deref, sync::Arc};

use cosmic::{
    app::{Core, Task},
    widget::{self, icon, nav_bar},
    Application, ApplicationExt,
};
use macaddr::MacAddr6;
use openscq30_lib::api::device::{DeviceDescriptor, OpenSCQ30Device};
use openscq30_storage::OpenSCQ30Database;

use crate::{
    add_device::{self, AddDeviceModel},
    device_selection::{self, DeviceSelectionModel},
    device_settings, fl,
};

pub struct AppModel {
    core: Core,
    screen: Screen,
    _nav: nav_bar::Model,
    dialog_page: Option<DialogPage>,
    database: OpenSCQ30Database,
}

#[derive(Debug, Clone)]
pub enum Message {
    DeviceSelectionScreen(device_selection::Message),
    AddDeviceScreen(add_device::Message),
    DeviceSettingsScreen(device_settings::Message),
    CloseDialog,
    RemovePairedDevice(MacAddr6),
    BackToDeviceSelection,
    ConnectToDeviceScreen(DebugOpenSCQ30Device),
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

pub enum Page {
    Page1,
}
enum DialogPage {
    RemoveDevice(MacAddr6),
}

enum Screen {
    DeviceSelection(device_selection::DeviceSelectionModel),
    AddDevice(add_device::AddDeviceModel),
    DeviceSettings(device_settings::DeviceSettingsModel),
}

impl Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;

    const APP_ID: &'static str = "com.oppzippy.OpenSCQ30";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let mut nav = nav_bar::Model::default();
        nav.insert()
            .text("page 1")
            .data::<Page>(Page::Page1)
            .activate();

        let database = OpenSCQ30Database::new().expect("database is required to run");
        let mut app = AppModel {
            core,
            _nav: nav,
            screen: Screen::DeviceSelection(DeviceSelectionModel::new(database.clone())),
            dialog_page: None,
            database,
        };
        let command = app.update_title();
        (app, command)
    }

    fn nav_model(&self) -> Option<&nav_bar::Model> {
        match &self.screen {
            Screen::DeviceSettings(model) => model.nav_model(),
            _ => None,
        }
    }

    fn header_start(&self) -> Vec<cosmic::Element<Self::Message>> {
        match self.screen {
            Screen::DeviceSelection(_) => Vec::new(),
            _ => vec![
                widget::button::icon(icon::from_name("go-previous-symbolic"))
                    .on_press(Message::BackToDeviceSelection)
                    .into(),
            ],
        }
    }

    fn view(&self) -> cosmic::Element<Self::Message> {
        match &self.screen {
            Screen::DeviceSelection(device_selection_model) => device_selection_model
                .view()
                .map(Message::DeviceSelectionScreen)
                .into(),
            Screen::AddDevice(add_device_model) => {
                add_device_model.view().map(Message::AddDeviceScreen).into()
            }
            Screen::DeviceSettings(device_settings_model) => device_settings_model
                .view()
                .map(Message::DeviceSettingsScreen)
                .into(),
        }
    }

    fn dialog(&self) -> Option<cosmic::Element<Self::Message>> {
        let dialog_page = self.dialog_page.as_ref()?;
        Some(match dialog_page {
            DialogPage::RemoveDevice(mac_address) => widget::dialog()
                .title(fl!("prompt-remove-device-title"))
                .body(fl!("prompt-remove-device", name = "TODO device name"))
                .icon(icon::from_name("dialog-warning-symbolic"))
                .primary_action(
                    widget::button::destructive(fl!("remove"))
                        .on_press(Message::RemovePairedDevice(*mac_address)),
                )
                .secondary_action(
                    widget::button::text(fl!("cancel")).on_press(Message::CloseDialog),
                )
                .into(),
        })
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::DeviceSelectionScreen(message) => {
                if let Screen::DeviceSelection(ref mut screen) = self.screen {
                    match screen.update(message) {
                        device_selection::Action::ConnectToDevice(paired_device) => {
                            return Task::future(async move {
                                let registry = paired_device
                                    .model
                                    .device_registry::<openscq30_lib::futures::TokioFutures>(
                                        Some(tokio::runtime::Handle::current()),
                                        true,
                                    )
                                    .await
                                    .expect("TODO error handling");
                                let device = registry
                                    .connect(paired_device.mac_address)
                                    .await
                                    .expect("TODO error handling");
                                cosmic::app::Message::App(Message::ConnectToDeviceScreen(
                                    DebugOpenSCQ30Device(device),
                                ))
                            })
                        }
                        device_selection::Action::RemoveDevice(mac_address) => {
                            self.dialog_page = Some(DialogPage::RemoveDevice(mac_address));
                        }
                        device_selection::Action::AddDevice => {
                            self.screen = Screen::AddDevice(AddDeviceModel::new())
                        }
                    }
                }
            }
            Message::AddDeviceScreen(message) => {
                if let Screen::AddDevice(ref mut screen) = self.screen {
                    match screen.update(message) {
                        add_device::Action::None => (),
                        add_device::Action::Task(task) => {
                            return task
                                .map(Message::AddDeviceScreen)
                                .map(cosmic::app::Message::App)
                        }
                        add_device::Action::AddDevice { model, descriptor } => {
                            self.database
                                .upsert_paired_device(&openscq30_storage::PairedDevice {
                                    name: descriptor.name().to_string(),
                                    mac_address: descriptor.mac_address(),
                                    model,
                                })
                                .expect("TODO error handling");
                            self.screen = Screen::DeviceSelection(DeviceSelectionModel::new(
                                self.database.clone(),
                            ));
                        }
                    }
                }
            }
            Message::DeviceSettingsScreen(message) => {
                if let Screen::DeviceSettings(ref mut screen) = self.screen {
                    match screen.update(message) {
                        device_settings::Action::Task(task) => {
                            return task
                                .map(Message::DeviceSettingsScreen)
                                .map(cosmic::app::Message::App)
                        }
                        device_settings::Action::None => (),
                    }
                }
            }
            Message::CloseDialog => self.dialog_page = None,
            Message::RemovePairedDevice(mac_address) => {
                self.database
                    .delete_paired_device(mac_address)
                    .expect("TODO error handling");
                if let Screen::DeviceSelection(ref mut screen) = self.screen {
                    self.dialog_page = None;
                    screen.refresh_paired_devices();
                }
            }
            Message::BackToDeviceSelection => {
                self.screen =
                    Screen::DeviceSelection(DeviceSelectionModel::new(self.database.clone()))
            }
            Message::ConnectToDeviceScreen(device) => {
                self.screen =
                    Screen::DeviceSettings(device_settings::DeviceSettingsModel::new(device));
            }
        }
        Task::none()
    }
}

impl AppModel {
    pub fn update_title(&mut self) -> Task<Message> {
        if let Some(id) = self.core.main_window_id() {
            self.set_header_title(fl!("openscq30"));
            self.set_window_title(fl!("openscq30"), id)
        } else {
            Task::none()
        }
    }
}
