use std::{marker::PhantomData, sync::Arc};

use async_trait::async_trait;
use tokio::sync::{mpsc, watch};

use crate::{
    api::{
        connection::{RfcommBackend, RfcommConnection},
        device::{GenericDeviceDescriptor, OpenSCQ30Device, OpenSCQ30DeviceRegistry},
        settings::{CategoryId, Setting, SettingId, Value},
    },
    device_utils,
    soundcore_device::{
        device::{Packet, packet_io_controller::PacketIOController},
        device_model::DeviceModel,
    },
    storage::OpenSCQ30Database,
};

use super::{
    modules::{
        ModuleCollection, ModuleCollectionSpawnPacketHandlerExt, sound_modes::AvailableSoundModes,
    },
    packets::{
        inbound::{InboundPacket, TryIntoInboundPacket},
        outbound::RequestStatePacket,
    },
    structures::{
        AgeRange, BasicHearId, CustomHearId, EqualizerConfiguration, Gender,
        InternalMultiButtonConfiguration, SoundModes, TwsStatus,
    },
};

pub struct SoundcoreDeviceRegistry<B: RfcommBackend, StateType, StateUpdatePacketType> {
    backend: B,
    database: Arc<OpenSCQ30Database>,
    device_model: DeviceModel,
    _state: PhantomData<StateType>,
    _state_update_packet: PhantomData<StateUpdatePacketType>,
}

impl<B: RfcommBackend, StateType, StateUpdatePacketType>
    SoundcoreDeviceRegistry<B, StateType, StateUpdatePacketType>
{
    pub fn new(backend: B, database: Arc<OpenSCQ30Database>, device_model: DeviceModel) -> Self {
        Self {
            backend,
            device_model,
            database,
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
    StateUpdatePacketType: InboundPacket + Into<StateType> + Send + Sync + 'static,
    Self: BuildDevice<B::ConnectionType, StateType, StateUpdatePacketType>,
{
    async fn devices(&self) -> crate::Result<Vec<GenericDeviceDescriptor>> {
        self.backend.devices().await.map(|descriptors| {
            descriptors
                .into_iter()
                .map(|d| GenericDeviceDescriptor::new(d.name, d.mac_address))
                .collect()
        })
    }

    async fn connect(
        &self,
        mac_address: macaddr::MacAddr6,
    ) -> crate::Result<Arc<dyn OpenSCQ30Device + Send + Sync>> {
        let connection = self
            .backend
            .connect(mac_address, |addr| {
                addr.into_iter()
                    .find(device_utils::is_soundcore_vendor_rfcomm_uuid)
                    .unwrap_or(device_utils::RFCOMM_UUID)
            })
            .await?;
        let mut builder =
            SoundcoreDeviceBuilder::new(self.database.clone(), connection, self.device_model)
                .await?;
        Self::build_device(&mut builder).await;
        Ok(Arc::new(builder.build()))
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
    _state_update: PhantomData<StateUpdatePacketType>,
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceBuilder<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + Send + Sync + 'static,
    StateType: Send + Sync + Clone + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
{
    pub async fn new(
        database: Arc<OpenSCQ30Database>,
        connection: ConnectionType,
        device_model: DeviceModel,
    ) -> crate::Result<Self> {
        let (packet_io_controller, packet_receiver) =
            PacketIOController::<ConnectionType>::new(Arc::new(connection)).await?;
        let packet_io_controller = Arc::new(packet_io_controller);
        let state_update_packet: StateUpdatePacketType = packet_io_controller
            .send(&RequestStatePacket::new().into())
            .await?
            .try_into_inbound_packet()?;
        let (state_sender, _) = watch::channel::<StateType>(state_update_packet.into());

        let module_collection = ModuleCollection::<StateType>::default();

        Ok(Self {
            device_model,
            state_sender,
            packet_io_controller,
            module_collection,
            database,
            packet_receiver,
            _state_update: PhantomData,
        })
    }

    pub fn build(
        self,
    ) -> SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType> {
        SoundcoreDeviceTemplate::new(
            self.packet_io_controller,
            self.state_sender,
            self.module_collection,
            self.packet_receiver,
            self.device_model,
        )
    }

    pub fn module_collection(&mut self) -> &mut ModuleCollection<StateType> {
        &mut self.module_collection
    }

    pub fn sound_modes(&mut self, available_sound_modes: AvailableSoundModes)
    where
        StateType: AsRef<SoundModes> + AsMut<SoundModes>,
    {
        self.module_collection
            .add_sound_modes(self.packet_io_controller.clone(), available_sound_modes);
    }

    pub async fn mono_equalizer(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration> + AsMut<EqualizerConfiguration>,
    {
        self.module_collection
            .add_equalizer(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                false,
            )
            .await;
    }

    pub async fn stereo_equalizer(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration> + AsMut<EqualizerConfiguration>,
    {
        self.module_collection
            .add_equalizer(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                true,
            )
            .await;
    }

    pub async fn mono_equalizer_with_drc(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration> + AsMut<EqualizerConfiguration>,
    {
        self.module_collection
            .add_equalizer_with_drc(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                false,
            )
            .await
    }

    pub async fn stereo_equalizer_with_drc(&mut self)
    where
        StateType: AsRef<EqualizerConfiguration> + AsMut<EqualizerConfiguration>,
    {
        self.module_collection
            .add_equalizer_with_drc(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
                true,
            )
            .await
    }

    pub async fn stereo_equalizer_with_basic_hear_id(&mut self)
    where
        StateType: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration>,
        StateType: AsRef<BasicHearId> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_basic_hear_id(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
            )
            .await
    }

    pub async fn stereo_equalizer_with_custom_hear_id(&mut self)
    where
        StateType: AsMut<EqualizerConfiguration> + AsRef<EqualizerConfiguration>,
        StateType: AsRef<CustomHearId> + AsRef<Gender> + AsRef<AgeRange>,
    {
        self.module_collection
            .add_equalizer_with_custom_hear_id(
                self.packet_io_controller.clone(),
                self.database.clone(),
                self.device_model,
            )
            .await
    }

    pub fn button_configuration(&mut self)
    where
        StateType:
            AsRef<InternalMultiButtonConfiguration> + AsMut<InternalMultiButtonConfiguration>,
        StateType: AsRef<TwsStatus>,
    {
        self.module_collection
            .add_button_configuration(self.packet_io_controller.clone())
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
    _packet_io_controller: Arc<PacketIOController<ConnectionType>>,
    _state_update: PhantomData<StateUpdatePacketType>,
}

impl<ConnectionType, StateType, StateUpdatePacketType>
    SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
    StateType: Clone + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType>,
{
    fn new(
        packet_io_controller: Arc<PacketIOController<ConnectionType>>,
        state_sender: watch::Sender<StateType>,
        module_collection: ModuleCollection<StateType>,
        packet_receiver: mpsc::Receiver<Packet>,
        device_model: DeviceModel,
    ) -> Self {
        let module_collection = Arc::new(module_collection);
        module_collection.spawn_packet_handler(state_sender.clone(), packet_receiver);

        Self {
            device_model,
            state_sender,
            _packet_io_controller: packet_io_controller,
            module_collection,
            _state_update: PhantomData,
        }
    }
}

#[async_trait]
impl<ConnectionType, StateType, StateUpdatePacketType> OpenSCQ30Device
    for SoundcoreDeviceTemplate<ConnectionType, StateType, StateUpdatePacketType>
where
    ConnectionType: RfcommConnection + 'static + Send + Sync,
    StateType: Clone + Send + Sync + 'static,
    StateUpdatePacketType: InboundPacket + Into<StateType> + Send + Sync,
{
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

    async fn set_setting_values(
        &self,
        setting_values: Vec<(SettingId, Value)>,
    ) -> crate::Result<()> {
        let mut target_state = self.state_sender.borrow().clone();
        for (setting_id, value) in setting_values {
            self.module_collection
                .setting_manager
                .set(&mut target_state, &setting_id, value)
                .await
                .unwrap()?;
        }
        for modifier in &self.module_collection.state_modifiers {
            modifier
                .move_to_state(&self.state_sender, &target_state)
                .await?;
        }
        Ok(())
    }
}
