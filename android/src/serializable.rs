use std::str::FromStr;

use openscq30_lib::{
    api::{connection, settings},
    devices, storage,
};

pub struct MacAddr6(pub macaddr::MacAddr6);
uniffi::custom_type!(MacAddr6, String, {
    try_lift: |val| Ok(MacAddr6(macaddr::MacAddr6::from_str(&val)?)),
    lower: |val| val.0.to_string(),
});

pub struct Uuid(pub uuid::Uuid);
uniffi::custom_type!(Uuid, String, {
    try_lift: |val| Ok(Uuid(uuid::Uuid::from_str(&val)?)),
    lower: |val| val.0.to_string(),
});

pub struct PairedDevice(pub storage::PairedDevice);
uniffi::custom_type!(PairedDevice, String, {
    try_lift: |json| Ok(PairedDevice(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct DeviceDescriptor(pub connection::DeviceDescriptor);
uniffi::custom_type!(DeviceDescriptor, String, {
    try_lift: |json| Ok(DeviceDescriptor(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct DeviceModel(pub devices::DeviceModel);
uniffi::custom_type!(DeviceModel, String, {
    try_lift: |json| Ok(DeviceModel(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct CategoryId(pub settings::CategoryId);
uniffi::custom_type!(CategoryId, String, {
    try_lift: |json| Ok(CategoryId(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

#[derive(Hash, PartialEq, Eq)]
pub struct SettingId(pub settings::SettingId);
uniffi::custom_type!(SettingId, String, {
    try_lift: |json| Ok(SettingId(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct Setting(pub settings::Setting);
uniffi::custom_type!(Setting, String, {
    try_lift: |json| Ok(Setting(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct Value(pub settings::Value);
uniffi::custom_type!(Value, String, {
    try_lift: |json| Ok(Value(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});

pub struct QuickPreset(pub openscq30_lib::api::quick_presets::QuickPreset);
uniffi::custom_type!(QuickPreset, String, {
    try_lift: |json| Ok(QuickPreset(serde_json::from_str(&json)?)),
    lower: |val| serde_json::to_string(&val.0).expect("json serialization shouldn't fail"),
});
