mod equalizer;
mod select;

use std::borrow::Cow;

use flatbuffers::WIPOffset;
use openscq30_lib::api::settings;

use crate::{
    FromFlatbufferError,
    generated::{self, setting::SettingUnion},
    prelude::*,
};

fn to_setting<'fbb, T>(
    fbb: &mut flatbuffers::FlatBufferBuilder<'fbb>,
    ty: generated::setting::SettingUnion,
    setting: WIPOffset<T>,
) -> WIPOffset<generated::setting::Setting<'fbb>> {
    generated::setting::Setting::create(
        fbb,
        &generated::setting::SettingArgs {
            inner_type: ty,
            inner: Some(setting.as_union_value()),
        },
    )
}

impl<'a> AsFlatbufferExt<'a> for settings::Setting {
    type Output = generated::setting::Setting<'a>;

    fn serialize_to_flatbuffer(
        &'a self,
        fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> WIPOffset<Self::Output> {
        match self {
            settings::Setting::Toggle { value } => {
                let inner = generated::setting::SettingToggle::create(
                    fbb,
                    &generated::setting::SettingToggleArgs { value: *value },
                );
                to_setting(fbb, SettingUnion::SettingToggle, inner)
            }
            settings::Setting::I32Range { setting, value } => {
                let inner = generated::setting::SettingI32Range::create(
                    fbb,
                    &generated::setting::SettingI32RangeArgs {
                        start: *setting.range.start(),
                        end: *setting.range.end(),
                        step: setting.step,
                        value: *value,
                    },
                );
                to_setting(fbb, SettingUnion::SettingI32Range, inner)
            }
            settings::Setting::Select { setting, value } => {
                let setting_offset = setting.serialize_to_flatbuffer(fbb);
                let value_offset = fbb.create_string(value);
                let inner = generated::setting::SettingSelect::create(
                    fbb,
                    &generated::setting::SettingSelectArgs {
                        select: Some(setting_offset),
                        value: Some(value_offset),
                    },
                );
                to_setting(fbb, SettingUnion::SettingSelect, inner)
            }
            settings::Setting::OptionalSelect { setting, value } => {
                let setting_offset = setting.serialize_to_flatbuffer(fbb);
                let value_offset = value.as_ref().map(|v| fbb.create_string(v.as_ref()));
                let inner = generated::setting::SettingOptionalSelect::create(
                    fbb,
                    &generated::setting::SettingOptionalSelectArgs {
                        select: Some(setting_offset),
                        value: value_offset,
                    },
                );
                to_setting(fbb, SettingUnion::SettingOptionalSelect, inner)
            }
            settings::Setting::ModifiableSelect { setting, value } => {
                let setting_offset = setting.serialize_to_flatbuffer(fbb);
                let value_offset = value.as_ref().map(|v| fbb.create_string(v.as_ref()));
                let inner = generated::setting::SettingModifiableSelect::create(
                    fbb,
                    &generated::setting::SettingModifiableSelectArgs {
                        select: Some(setting_offset),
                        value: value_offset,
                    },
                );
                to_setting(fbb, SettingUnion::SettingModifiableSelect, inner)
            }
            settings::Setting::Equalizer { setting, values } => {
                let setting_offset = setting.serialize_to_flatbuffer(fbb);
                let value_offset = fbb.create_vector(values);
                let inner = generated::setting::SettingEqualizer::create(
                    fbb,
                    &generated::setting::SettingEqualizerArgs {
                        equalizer: Some(setting_offset),
                        value: Some(value_offset),
                    },
                );
                to_setting(fbb, SettingUnion::SettingEqualizer, inner)
            }
            settings::Setting::Information {
                text,
                translated_text,
            } => {
                let text_offset = fbb.create_string(text);
                let translated_text_offset = fbb.create_string(translated_text);
                let inner = generated::setting::SettingInformation::create(
                    fbb,
                    &generated::setting::SettingInformationArgs {
                        text: Some(text_offset),
                        translated_text: Some(translated_text_offset),
                    },
                );
                to_setting(fbb, SettingUnion::SettingInformation, inner)
            }
        }
    }
}

impl FromFlatbufferExt for settings::Setting {
    fn from_flatbuffer(buffer: &[u8]) -> Result<Self, FromFlatbufferError> {
        const MISSING_INNER: FromFlatbufferError = FromFlatbufferError::MissingRequiredValue {
            name: "Setting.inner",
        };
        let setting = flatbuffers::root::<generated::setting::Setting>(buffer)?;

        type Parser =
            fn(generated::setting::Setting) -> Result<settings::Setting, FromFlatbufferError>;
        const PARSERS: &[(generated::setting::SettingUnion, Parser)] = &[
            (SettingUnion::NONE, |_| {
                Err(FromFlatbufferError::MissingRequiredValue {
                    name: "Value.inner_type",
                })
            }),
            (SettingUnion::SettingToggle, |setting| {
                Ok(settings::Setting::Toggle {
                    value: setting
                        .inner_as_setting_toggle()
                        .ok_or(MISSING_INNER)?
                        .value(),
                })
            }),
            (SettingUnion::SettingI32Range, |setting| {
                let setting_i32_range =
                    setting.inner_as_setting_i32_range().ok_or(MISSING_INNER)?;
                Ok(settings::Setting::I32Range {
                    setting: settings::Range {
                        range: setting_i32_range.start()..=setting_i32_range.end(),
                        step: setting_i32_range.step(),
                    },
                    value: setting_i32_range.value(),
                })
            }),
            (SettingUnion::SettingSelect, |setting| {
                let setting_select = setting.inner_as_setting_select().ok_or(MISSING_INNER)?;
                Ok(settings::Setting::Select {
                    setting: setting_select.select().into(),
                    value: setting_select.value().to_owned().into(),
                })
            }),
            (SettingUnion::SettingOptionalSelect, |setting| {
                let setting_optional_select = setting
                    .inner_as_setting_optional_select()
                    .ok_or(MISSING_INNER)?;
                Ok(settings::Setting::OptionalSelect {
                    setting: setting_optional_select.select().into(),
                    value: setting_optional_select
                        .value()
                        .map(ToOwned::to_owned)
                        .map(Cow::Owned),
                })
            }),
            (SettingUnion::SettingModifiableSelect, |setting| {
                let setting_modifiable_select = setting
                    .inner_as_setting_modifiable_select()
                    .ok_or(MISSING_INNER)?;
                Ok(settings::Setting::ModifiableSelect {
                    setting: setting_modifiable_select.select().into(),
                    value: setting_modifiable_select
                        .value()
                        .map(ToOwned::to_owned)
                        .map(Cow::Owned),
                })
            }),
            (SettingUnion::SettingEqualizer, |setting| {
                let setting_equalizer =
                    setting.inner_as_setting_equalizer().ok_or(MISSING_INNER)?;

                Ok(settings::Setting::Equalizer {
                    setting: setting_equalizer.equalizer().into(),
                    values: setting_equalizer.value().into_iter().collect(),
                })
            }),
            (SettingUnion::SettingInformation, |setting| {
                let setting_information = setting
                    .inner_as_setting_information()
                    .ok_or(MISSING_INNER)?;
                Ok(settings::Setting::Information {
                    text: setting_information.text().to_owned(),
                    translated_text: setting_information.translated_text().to_owned(),
                })
            }),
        ];

        // Since ENUM_VALUES uses constants rather than an enum, we can't use match to ensure
        // all variants are covered. static_assertions is used as a workaround.
        static_assertions::const_assert_eq!(
            PARSERS.len(),
            generated::setting::SettingUnion::ENUM_VALUES.len(),
        );

        PARSERS
            .iter()
            .find(|(key, _)| *key == setting.inner_type())
            .ok_or(FromFlatbufferError::InvalidUnionVariant {
                union_name: "SettingUnion",
                variant: setting.inner_type().0,
            })
            .map(|(_, parser)| parser(setting))?
    }
}
