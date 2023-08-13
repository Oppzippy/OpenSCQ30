use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PacketType {
    SoundModeUpdate,
    SetSoundModeOk,
    SetEqualizerOk,
    StateUpdate,
    FirmwareVersionUpdate,
    SetEqualizerWithDrcOk,
    BatteryLevelUpdate,
    BatteryChargingUpdate,
    TwsStatusUpdate,
}
