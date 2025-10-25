use std::{marker::PhantomData, pin::Pin, sync::Arc};

use async_trait::async_trait;
use openscq30_i18n::Translate;
use openscq30_lib_has::{Has, MaybeHas};
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
    devices::{
        DeviceModel,
        soundcore::{
            self,
            common::{
                modules::{
                    button_configuration::ButtonConfigurationSettings,
                    reset_button_configuration::ResetButtonConfigurationPending,
                },
                packet::{self, PacketIOController, outbound::ToPacket},
                state::Update,
                structures::{
                    AutoPowerOff, TouchTone, button_configuration::ButtonStatusCollection,
                },
            },
        },
    },
    storage::OpenSCQ30Database,
};

use super::{
    modules::{
        ModuleCollection, ModuleCollectionSpawnPacketHandlerExt, sound_modes::AvailableSoundModes,
    },
    packet::{
        inbound::{FromPacketBody, TryToPacket},
        outbound::RequestState,
    },
    structures::{
        AgeRange, AmbientSoundModeCycle, BasicHearId, CustomHearId, DualBattery,
        DualFirmwareVersion, EqualizerConfiguration, FirmwareVersion, Gender, SerialNumber,
        SingleBattery, SoundModes, TwsStatus,
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
    StateUpdate: FromPacketBody + Default + Into<State>,
{
    let state_update_packet: StateUpdate = packet_io
        .send_with_response(&RequestState::default().to_packet())
        .await?
        .try_to_packet()
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
    StateUpdatePacketType: FromPacketBody + Send + Sync + 'static,
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
    packet_receiver: mpsc::Receiver<packet::Inbound>,
    change_notify: watch::Sender<()>,
    _state_update: PhantomData<StateUpdatePacketType>,
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Send + Sync + Clone + 'static,
    StateUpdatePacketType: FromPacketBody,
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
        StateType: Has<SoundModes>,
    {
        self.module_collection
            .add_sound_modes(self.packet_io_controller.clone(), available_sound_modes);
    }

    pub async fn equalizer<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>>,
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

    pub async fn equalizer_tws<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>> + Has<TwsStatus>,
    {
        self.module_collection
            .add_equalizer_tws(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_drc<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>>,
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

    pub async fn equalizer_with_drc_tws<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>> + Has<TwsStatus>,
    {
        self.module_collection
            .add_equalizer_with_drc_tws(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_basic_hear_id_tws<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>>
            + Has<TwsStatus>
            + Has<BasicHearId<C, B>>
            + Has<Gender>
            + Has<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_basic_hear_id_tws(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub async fn equalizer_with_custom_hear_id_tws<const C: usize, const B: usize>(&mut self)
    where
        StateType: Has<EqualizerConfiguration<C, B>>
            + Has<TwsStatus>
            + Has<CustomHearId<C, B>>
            + Has<Gender>
            + Has<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_custom_hear_id_tws(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                self.change_notify.clone(),
            )
            .await;
    }

    pub fn button_configuration<const NUM_BUTTONS: usize, const NUM_PRESS_KINDS: usize>(
        &mut self,
        settings: &'static ButtonConfigurationSettings<NUM_BUTTONS, NUM_PRESS_KINDS>,
    ) where
        StateType: Has<ButtonStatusCollection<NUM_BUTTONS>>
            + Has<TwsStatus>
            + Has<ResetButtonConfigurationPending>,
    {
        self.module_collection
            .add_button_configuration(self.packet_io_controller.clone(), settings);
    }

    pub fn ambient_sound_mode_cycle(&mut self)
    where
        StateType: Has<AmbientSoundModeCycle> + Has<ResetButtonConfigurationPending>,
    {
        self.module_collection
            .add_ambient_sound_mode_cycle(self.packet_io_controller.clone());
    }

    pub fn reset_button_configuration<ButtonConfigurationPacketType>(
        &mut self,
        request_button_configuration_packet: packet::Outbound,
    ) where
        StateType: Has<ResetButtonConfigurationPending> + Update<ButtonConfigurationPacketType>,
        ButtonConfigurationPacketType: FromPacketBody,
    {
        self.module_collection
            .add_reset_button_configuration::<ConnectionType, ButtonConfigurationPacketType>(
                self.packet_io_controller.clone(),
                request_button_configuration_packet,
            );
    }

    pub fn single_battery(&mut self)
    where
        StateType: Has<SingleBattery>,
    {
        self.module_collection.add_single_battery();
    }

    pub fn dual_battery(&mut self, max_level: u8)
    where
        StateType: Has<DualBattery>,
    {
        self.module_collection.add_dual_battery(max_level);
    }

    pub fn serial_number_and_firmware_version(&mut self)
    where
        StateType: Has<SerialNumber> + Has<FirmwareVersion>,
    {
        self.module_collection
            .add_serial_number_and_firmware_version();
    }

    pub fn serial_number_and_dual_firmware_version(&mut self)
    where
        StateType: Has<SerialNumber> + Has<DualFirmwareVersion>,
    {
        self.module_collection
            .add_serial_number_and_dual_firmware_version();
    }

    pub fn tws_status(&mut self)
    where
        StateType: Has<TwsStatus>,
    {
        self.module_collection.add_tws_status();
    }

    pub fn auto_power_off<Duration>(&mut self, durations: &'static [Duration])
    where
        StateType: MaybeHas<AutoPowerOff>,
        Duration: Translate + Send + Sync + 'static,
        &'static str: for<'a> From<&'a Duration>,
    {
        self.module_collection
            .add_auto_power_off(self.packet_io_controller.clone(), durations);
    }

    pub fn touch_tone(&mut self)
    where
        StateType: Has<TouchTone>,
    {
        self.module_collection
            .add_touch_tone(self.packet_io_controller.clone());
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
    StateUpdatePacketType: FromPacketBody,
{
    async fn new(
        packet_io_controller: Arc<PacketIOController<ConnectionType>>,
        state_sender: watch::Sender<StateType>,
        module_collection: ModuleCollection<StateType>,
        packet_receiver: mpsc::Receiver<packet::Inbound>,
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
    StateUpdatePacketType: FromPacketBody + Send + Sync,
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
        tracing::debug!("set values: {setting_values:?}");
        self.module_collection
            .set_setting_values(&self.state_sender, setting_values)
            .await
    }
}

#[cfg(test)]
pub mod test_utils {
    use std::{collections::HashMap, time::Duration};

    use macaddr::MacAddr6;
    use nom_language::error::VerboseError;

    use crate::{
        devices::soundcore::common::packet::{
            self, Command, inbound::FromPacketBody, outbound::ToPacket,
        },
        mock::rfcomm::{MockRfcommBackend, MockRfcommConnection},
    };

    use super::*;

    pub struct TestSoundcoreDevice {
        device: Arc<dyn OpenSCQ30Device + Send + Sync>,
        inbound_sender: mpsc::Sender<Vec<u8>>,
        outbound_receiver: mpsc::Receiver<Vec<u8>>,
    }

    impl TestSoundcoreDevice {
        pub async fn new<StateType, StateUpdatePacketType>(
            constructor: fn(
                MockRfcommBackend,
                Arc<OpenSCQ30Database>,
                DeviceModel,
            ) -> SoundcoreDeviceRegistry<
                MockRfcommBackend,
                StateType,
                StateUpdatePacketType,
            >,
            device_model: DeviceModel,
        ) -> Self
        where
            StateType: Clone + Send + Sync + 'static,
            StateUpdatePacketType: FromPacketBody
                + ToPacket<DirectionMarker = packet::InboundMarker>
                + Clone
                + Send
                + Sync
                + Default
                + 'static,
            SoundcoreDeviceRegistry<MockRfcommBackend, StateType, StateUpdatePacketType>:
                BuildDevice<MockRfcommConnection, StateType, StateUpdatePacketType>,
        {
            Self::new_with_packet_responses(
                constructor,
                device_model,
                HashMap::from([
                    (
                        Command([1, 1]),
                        StateUpdatePacketType::default().to_packet(),
                    ),
                    (
                        packet::inbound::SerialNumberAndFirmwareVersion::COMMAND,
                        packet::inbound::SerialNumberAndFirmwareVersion::default().to_packet(),
                    ),
                ]),
            )
            .await
        }

        pub async fn new_with_packet_responses<StateType, StateUpdatePacketType>(
            constructor: fn(
                MockRfcommBackend,
                Arc<OpenSCQ30Database>,
                DeviceModel,
            ) -> SoundcoreDeviceRegistry<
                MockRfcommBackend,
                StateType,
                StateUpdatePacketType,
            >,
            device_model: DeviceModel,
            packet_responses: HashMap<Command, packet::Inbound>,
        ) -> Self
        where
            StateType: Clone + Send + Sync + 'static,
            StateUpdatePacketType:
                FromPacketBody + ToPacket + Clone + Send + Sync + Default + 'static,
            SoundcoreDeviceRegistry<MockRfcommBackend, StateType, StateUpdatePacketType>:
                BuildDevice<MockRfcommConnection, StateType, StateUpdatePacketType>,
        {
            let (inbound_sender, inbound_receiver) = mpsc::channel(100);
            let (outbound_sender, mut outbound_receiver) = mpsc::channel(100);
            let database = Arc::new(OpenSCQ30Database::new_in_memory().await.unwrap());

            let registry = constructor(
                MockRfcommBackend::new(inbound_receiver, outbound_sender),
                database,
                device_model,
            );

            // spawn a future to connect to the device rather than to handle packets so that we don't have to move
            // outbound_receiver into the future and back out when it's done
            let (device_sender, mut device_receiver) = mpsc::channel(1);
            tokio::spawn(async move {
                device_sender
                    .try_send(registry.connect(MacAddr6::nil()).await.unwrap())
                    .unwrap();
            });

            // respond to packets until the device is ready
            let device = loop {
                select! {
                    maybe_packet = outbound_receiver.recv() => {
                        if let Some(packet) = maybe_packet {
                            let command = Command(packet[5..7].try_into().unwrap());
                            inbound_sender
                                .send(packet_responses.get(&command).unwrap().bytes())
                                .await
                                .unwrap();
                        }
                    }
                    device = device_receiver.recv() => {
                        break device.unwrap();
                    }
                }
            };

            Self {
                device,
                inbound_sender,
                outbound_receiver,
            }
        }

        pub fn assert_setting_values(
            &self,
            expected_values: impl IntoIterator<Item = (SettingId, Value)>,
        ) {
            for (setting_id, expected) in expected_values {
                let setting = self
                    .device
                    .setting(&setting_id)
                    .expect(&format!("{setting_id} returned None"));
                assert_eq!(
                    Value::from(setting),
                    expected,
                    "{setting_id} should be {expected:?}"
                );
            }
        }

        pub async fn assert_set_settings_response_unordered(
            &mut self,
            settings: Vec<(SettingId, Value)>,
            expected_packets: Vec<packet::Outbound>,
        ) {
            let mut expected_packets_bytes = expected_packets
                .iter()
                .map(|expected| expected.bytes())
                .collect::<Vec<_>>();
            let mut sent_packets_bytes = self.set_settings_and_gather_sent_packets(settings).await;
            expected_packets_bytes.sort();
            sent_packets_bytes.sort();

            assert_eq!(sent_packets_bytes, expected_packets_bytes);
        }

        pub async fn assert_set_settings_response(
            &mut self,
            settings: Vec<(SettingId, Value)>,
            expected_packets: Vec<packet::Outbound>,
        ) {
            let expected_packets_bytes = expected_packets
                .iter()
                .map(|expected| expected.bytes())
                .collect::<Vec<_>>();
            let sent_packets_bytes = self.set_settings_and_gather_sent_packets(settings).await;
            assert_eq!(sent_packets_bytes, expected_packets_bytes);
        }

        async fn set_settings_and_gather_sent_packets(
            &mut self,
            settings: Vec<(SettingId, Value)>,
        ) -> Vec<Vec<u8>> {
            self.clear_sent_packets().await;
            let device = self.device.clone();
            let join_handle = tokio::spawn(async move {
                device.set_setting_values(settings).await.unwrap();
            });

            let mut sent_packets = Vec::new();
            select! {
                 _ = self.gather_sent_packets(&mut sent_packets) => (),
                 _ = join_handle => (),
            };
            sent_packets
        }

        async fn clear_sent_packets(&mut self) {
            // gather packets and discard return value
            // start_paused should be set to true to prevent this from taking forever
            _ = tokio::time::timeout(
                Duration::from_secs(30),
                self.gather_sent_packets(&mut Vec::new()),
            )
            .await;
        }

        async fn gather_sent_packets(&mut self, sent_packets: &mut Vec<Vec<u8>>) {
            loop {
                if let Some(bytes) = self.outbound_receiver.recv().await {
                    let packet = packet::Outbound::take::<VerboseError<_>>(&bytes).unwrap().1;
                    self.ack(&packet).await;
                    sent_packets.push(bytes);
                } else {
                    break;
                }
            }
        }

        async fn ack(&self, packet: &packet::Outbound) {
            // for cancel safety
            let permit = self.inbound_sender.reserve().await.unwrap();
            permit.send(packet.ack().bytes());
        }
    }
}
