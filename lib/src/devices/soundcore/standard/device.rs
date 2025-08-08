use std::{marker::PhantomData, pin::Pin, sync::Arc};

use async_trait::async_trait;
use tokio::{
    select,
    sync::{Semaphore, mpsc, watch},
};

use crate::{
    api::{
        connection::{ConnectionDescriptor, ConnectionStatus, RfcommBackend, RfcommConnection},
        device::{self, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
        settings::{CategoryId, Setting, SettingId, Value},
    },
    devices::{DeviceModel, soundcore},
    storage::OpenSCQ30Database,
};

use super::{
    modules::{
        ModuleCollection, ModuleCollectionSpawnPacketHandlerExt, sound_modes::AvailableSoundModes,
    },
    packet::{
        Packet,
        inbound::{InboundPacket, TryIntoInboundPacket},
        outbound::RequestStatePacket,
        packet_io_controller::PacketIOController,
    },
    structures::{
        AgeRange, AmbientSoundModeCycle, BasicHearId, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, FirmwareVersion, Gender,
        MultiButtonConfiguration, SerialNumber, SingleBattery, SoundModes, TwsStatus,
    },
};

type FetchStateFn<ConnectionType, StateType> = Box<
    dyn Fn(
            Arc<PacketIOController<ConnectionType>>,
        ) -> Pin<Box<dyn Future<Output = device::Result<StateType>> + Send>>
        + Send
        + Sync,
>;

pub async fn fetch_state_from_state_update_packet<C, State, StateUpdate>(
    packet_io: Arc<PacketIOController<C>>,
) -> device::Result<State>
where
    C: RfcommConnection,
    StateUpdate: InboundPacket + Default + Into<State>,
{
    let state_update_packet: StateUpdate = packet_io
        .send_with_response(&RequestStatePacket::new().into())
        .await?
        .try_into_inbound_packet()
        .map_err(|err| device::Error::other(err))?;
    Ok(state_update_packet.into())
}

pub struct SoundcoreDeviceRegistry<B: RfcommBackend, StateType, StateUpdatePacketType> {
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
    fetch_state: FetchStateFn<B::ConnectionType, StateType>,
    _state: PhantomData<StateType>,
    _state_update_packet: PhantomData<StateUpdatePacketType>,
}

impl<B: RfcommBackend, StateType, StateUpdatePacketType>
    SoundcoreDeviceRegistry<B, StateType, StateUpdatePacketType>
{
    pub fn new(
        backend: B,
        database: Arc<OpenSCQ30Database>,
        device_model: DeviceModel,
        fetch_state: FetchStateFn<B::ConnectionType, StateType>,
    ) -> Self {
        Self {
            backend,
            device_model,
            database,
            fetch_state,
            _state: PhantomData,
            _state_update_packet: PhantomData,
        }
    }
}

#[async_trait]
impl<B, StateType, StateUpdatePacketType> OpenSCQ30DeviceRegistry
    for SoundcoreDeviceRegistry<B, StateType, StateUpdatePacketType>
where
    B: RfcommBackend + 'static + Send + Sync,
    StateType: Clone + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Send + Sync + 'static,
    Self: BuildDevice<B::ConnectionType, StateType, StateUpdatePacketType>,
{
    async fn devices(&self) -> device::Result<Vec<ConnectionDescriptor>> {
        self.backend
            .devices()
            .await
            .map(|descriptors| {
                descriptors
                    .into_iter()
                    .map(|d| ConnectionDescriptor {
                        name: d.name,
                        mac_address: d.mac_address,
                    })
                    .collect()
            })
            .map_err(Into::into)
    }

    async fn connect(
        &self,
        mac_address: macaddr::MacAddr6,
    ) -> device::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        let connection = self
            .backend
            .connect(mac_address, |addr| {
                addr.into_iter()
                    .find(soundcore::is_soundcore_vendor_rfcomm_uuid)
                    .unwrap_or(soundcore::RFCOMM_UUID)
            })
            .await?;
        let mut builder = SoundcoreDeviceBuilder::new(
            self.database.clone(),
            connection,
            self.device_model,
            &self.fetch_state,
        )
        .await?;
        Self::build_device(&mut builder).await;
        Ok(Arc::new(builder.build().await))
    }
}

pub trait BuildDevice<ConnectionType, StateType, StateUpdateType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    StateType: Clone + Send + Sync,
{
    fn build_device(
        builder: &mut SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdateType>,
    ) -> impl Future<Output = ()> + Send;
}

pub struct SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
{
    device_model: DeviceModel,
    state_sender: watch::Sender<StateType>,
    module_collection: ModuleCollection<StateType>,
    packet_io_controller: Arc<PacketIOController<ConnectionType>>,
    database: Arc<OpenSCQ30Database>,
    packet_receiver: mpsc::Receiver<Packet>,
    change_notify: watch::Sender<()>,
    _state_update: PhantomData<StateUpdatePacketType>,
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Send + Sync + Clone + 'static,
    StateUpdatePacketType: InboundPacket,
{
    pub async fn new(
        database: Arc<OpenSCQ30Database>,
        connection: ConnectionType,
        device_model: DeviceModel,
        fetch_state: &FetchStateFn<ConnectionType, StateType>,
    ) -> device::Result<Self> {
        let (packet_io_controller, packet_receiver) =
            PacketIOController::<ConnectionType>::new(Arc::new(connection)).await?;
        let packet_io_controller = Arc::new(packet_io_controller);
        let state = fetch_state(packet_io_controller.clone()).await?;
        let (state_sender, _) = watch::channel::<StateType>(state);

        let module_collection = ModuleCollection::<StateType>::default();

        Ok(Self {
            device_model,
            state_sender,
            packet_io_controller,
            module_collection,
            database,
            packet_receiver,
            _state_update: PhantomData,
            change_notify: watch::channel(()).0,
        })
    }

    pub async fn build(
        self,
    ) -> SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType> {
        SoundcoreDeviceTemplate::new(
            self.packet_io_controller,
            self.state_sender,
            self.module_collection,
            self.packet_receiver,
            self.device_model,
            self.change_notify.subscribe(),
        )
        .await
    }

    pub fn module_collection(&mut self) -> &mut ModuleCollection<StateType> {
        &mut self.module_collection
    }

    pub fn packet_io_controller(&mut self) -> &Arc<PacketIOController<ConnectionType>> {
        &self.packet_io_controller
    }

    pub fn sound_modes(&mut self, available_sound_modes: AvailableSoundModes)
    where
        StateType: AsRef<SoundModes> + AsMut<SoundModes>,
    {
        self.module_collection
            .add_sound_modes(self.packet_io_controller.clone(), available_sound_modes);
    }

    pub async fn equalizer<const C: usize, const B: usize>(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration<C, B>> + AsMut<EqualizerConfiguration<C, B>>,
    {
        self.module_collection
            .add_equalizer(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_drc<const C: usize, const B: usize>(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration<C, B>> + AsMut<EqualizerConfiguration<C, B>>,
    {
        self.module_collection
            .add_equalizer_with_drc(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_basic_hear_id<const C: usize, const B: usize>(&mut self)
    where
        StateType: AsMut<EqualizerConfiguration<C, B>> + AsRef<EqualizerConfiguration<C, B>>,
        StateType: AsRef<BasicHearId<C, B>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_basic_hear_id(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_custom_hear_id<const C: usize, const B: usize>(&mut self)
    where
        StateType: AsMut<EqualizerConfiguration<C, B>> + AsRef<EqualizerConfiguration<C, B>>,
        StateType: AsRef<CustomHearId<C, B>> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_custom_hear_id(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub fn button_configuration(&mut self)
    where
        StateType: AsRef<MultiButtonConfiguration> + AsMut<MultiButtonConfiguration>,
        StateType: AsRef<TwsStatus>,
    {
        self.module_collection
            .add_button_configuration(self.packet_io_controller.clone());
    }

    pub fn ambient_sound_mode_cycle(&mut self)
    where
        StateType: AsRef<AmbientSoundModeCycle> + AsMut<AmbientSoundModeCycle>,
    {
        self.module_collection
            .add_ambient_sound_mode_cycle(self.packet_io_controller.clone());
    }

    pub fn single_battery(&mut self)
    where
        StateType: AsRef<SingleBattery> + AsMut<SingleBattery>,
    {
        self.module_collection.add_single_battery();
    }

    pub fn dual_battery(&mut self)
    where
        StateType: AsRef<DualBattery> + AsMut<DualBattery>,
    {
        self.module_collection.add_dual_battery();
    }

    pub fn serial_number_and_firmware_version(&mut self)
    where
        StateType: AsRef<SerialNumber>
            + AsMut<SerialNumber>
            + AsRef<FirmwareVersion>
            + AsMut<FirmwareVersion>,
    {
        self.module_collection
            .add_serial_number_and_firmware_version();
    }

    pub fn serial_number_and_dual_firmware_version(&mut self)
    where
        StateType: AsRef<SerialNumber>
            + AsMut<SerialNumber>
            + AsRef<DualFirmwareVersion>
            + AsMut<DualFirmwareVersion>,
    {
        self.module_collection
            .add_serial_number_and_dual_firmware_version();
    }

    pub fn tws_status(&mut self)
    where
        StateType: AsRef<TwsStatus> + AsMut<TwsStatus>,
    {
        self.module_collection.add_tws_status();
    }
}

pub struct SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    StateType: Clone + Send + Sync,
{
    device_model: DeviceModel,
    state_sender: watch::Sender<StateType>,
    module_collection: Arc<ModuleCollection<StateType>>,
    packet_io_controller: Arc<PacketIOController<ConnectionType>>,
    // TODO exit signal is necessary due to the PacketIOController Arc spaghetti.
    exit_signal: Arc<Semaphore>,
    change_notify: watch::Receiver<()>,
    _state_update: PhantomData<StateUpdatePacketType>,
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
    StateType: Clone + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket,
{
    async fn new(
        packet_io_controller: Arc<PacketIOController<ConnectionType>>,
        state_sender: watch::Sender<StateType>,
        module_collection: ModuleCollection<StateType>,
        packet_receiver: mpsc::Receiver<Packet>,
        device_model: DeviceModel,
        change_notify: watch::Receiver<()>,
    ) -> Self {
        let exit_signal = Arc::new(Semaphore::new(0));
        let module_collection = Arc::new(module_collection);
        module_collection
            .spawn_packet_handler(state_sender.clone(), packet_receiver, exit_signal.clone())
            .await;

        Self {
            device_model,
            state_sender,
            packet_io_controller,
            module_collection,
            exit_signal,
            _state_update: PhantomData,
            change_notify,
        }
    }
}

impl<ConnectionType, StateType, StateUpdatePacketType> Drop
    for SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync,
    StateType: Clone + Send + Sync,
{
    fn drop(&mut self) {
        self.exit_signal.close();
    }
}

#[async_trait]
impl<ConnectionType, StateType, StateUpdatePacketType> OpenSCQ30Device
    for SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
    StateType: Clone + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Send + Sync,
{
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.packet_io_controller.connection_status()
    }

    fn model(&self) -> DeviceModel {
        self.device_model
    }

    fn categories(&self) -> Vec<CategoryId> {
        self.module_collection.setting_manager.categories().to_vec()
    }

    fn settings_in_category(&self, category_id: &CategoryId) -> Vec<SettingId> {
        self.module_collection.setting_manager.category(category_id)
    }

    fn setting(&self, setting_id: &SettingId) -> Option<Setting> {
        let state = self.state_sender.borrow().to_owned();
        self.module_collection
            .setting_manager
            .get(&state, setting_id)
    }

    fn watch_for_changes(&self) -> watch::Receiver<()> {
        let mut receiver = self.state_sender.subscribe();
        let (change_sender, change_receiver) = watch::channel(());
        let mut change_notify = self.change_notify.clone();
        // receiver will close when self is dropped, so this will clean itself up
        tokio::spawn(async move {
            loop {
                select! {
                    result = receiver.changed() => if result.is_err() { return },
                    result = change_notify.changed() => if result.is_err() { return },
                }
                if change_sender.send(()).is_err() {
                    return;
                }
            }
        });
        change_receiver
    }

    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId, Value)>,
    ) -> device::Result<()> {
        self.module_collection
            .set_setting_values(&self.state_sender, setting_values)
            .await
    }
}
