use std::borrow::Cow;

use flatbuffers::{FlatBufferBuilder, WIPOffset};
use openscq30_lib::api::settings;

use crate::{
    FromFlatbufferError, create_string_vec,
    generated::{self, value::ValueUnion},
    prelude::*,
};

fn to_value<'fbb, T>(
    fbb: &mut flatbuffers::FlatBufferBuilder<'fbb>,
    ty: generated::value::ValueUnion,
    value: WIPOffset<T>,
) -> WIPOffset<generated::value::Value<'fbb>> {
    generated::value::Value::create(
        fbb,
        &generated::value::ValueArgs {
            inner_type: ty,
            inner: Some(value.as_union_value()),
        },
    )
}

impl<'a> AsFlatbufferExt<'a> for settings::Value {
    type Output = generated::value::Value<'a>;

    fn serialize_to_flatbuffer(&self, fbb: &mut FlatBufferBuilder<'a>) -> WIPOffset<Self::Output> {
        match self {
            settings::Value::Bool(value) => {
                let wrapper_offset = generated::value::Bool::create(
                    fbb,
                    &generated::value::BoolArgs { value: *value },
                );
                to_value(fbb, ValueUnion::Bool, wrapper_offset)
            }
            settings::Value::U16(value) => {
                let wrapper_offset = generated::value::U16::create(
                    fbb,
                    &generated::value::U16Args { value: *value },
                );
                to_value(fbb, ValueUnion::U16, wrapper_offset)
            }
            settings::Value::U16Vec(value) => {
                let value_offset = fbb.create_vector(value);
                let wrapper_offset = generated::value::U16Vec::create(
                    fbb,
                    &generated::value::U16VecArgs {
                        value: Some(value_offset),
                    },
                );
                to_value(fbb, ValueUnion::U16Vec, wrapper_offset)
            }
            settings::Value::OptionalU16(value) => {
                let wrapper_offset = generated::value::OptionalU16::create(
                    fbb,
                    &generated::value::OptionalU16Args { value: *value },
                );
                to_value(fbb, ValueUnion::OptionalU16, wrapper_offset)
            }
            settings::Value::I16Vec(value) => {
                let value_offset = fbb.create_vector(value);
                let wrapper_offset = generated::value::I16Vec::create(
                    fbb,
                    &generated::value::I16VecArgs {
                        value: Some(value_offset),
                    },
                );
                to_value(fbb, ValueUnion::I16Vec, wrapper_offset)
            }
            settings::Value::I32(value) => {
                let wrapper_offset = generated::value::I32::create(
                    fbb,
                    &generated::value::I32Args { value: *value },
                );
                to_value(fbb, ValueUnion::I32, wrapper_offset)
            }
            settings::Value::String(value) => {
                let string_offset = fbb.create_string(value.as_ref());
                let wrapper_offset = generated::value::String::create(
                    fbb,
                    &generated::value::StringArgs {
                        value: Some(string_offset),
                    },
                );
                to_value(fbb, ValueUnion::String, wrapper_offset)
            }
            settings::Value::StringVec(strings) => {
                let vec_offset = create_string_vec(fbb, strings);
                let wrapper_offset = generated::value::StringVec::create(
                    fbb,
                    &generated::value::StringVecArgs {
                        value: Some(vec_offset),
                    },
                );

                to_value(fbb, ValueUnion::StringVec, wrapper_offset)
            }
            settings::Value::OptionalString(value) => {
                let string_offset = value.as_ref().map(|v| fbb.create_string(v.as_ref()));
                let wrapper_offset = generated::value::OptionalString::create(
                    fbb,
                    &generated::value::OptionalStringArgs {
                        value: string_offset,
                    },
                );
                to_value(fbb, ValueUnion::OptionalString, wrapper_offset)
            }
        }
    }
}

impl FromFlatbufferExt for settings::Value {
    fn from_flatbuffer(buffer: &[u8]) -> Result<Self, FromFlatbufferError> {
        const MISSING_INNER: FromFlatbufferError = FromFlatbufferError::MissingRequiredValue {
            name: "Value.inner",
        };
        let value = flatbuffers::root::<generated::value::Value>(buffer)?;

        type Parser = fn(generated::value::Value) -> Result<settings::Value, FromFlatbufferError>;
        const PARSERS: &[(generated::value::ValueUnion, Parser)] = &[
            (ValueUnion::NONE, |_| {
                Err(FromFlatbufferError::MissingRequiredValue {
                    name: "Value.inner_type",
                })
            }),
            (ValueUnion::Bool, |value| {
                Ok(value.inner_as_bool().ok_or(MISSING_INNER)?.value().into())
            }),
            (ValueUnion::U16, |value| {
                Ok(value.inner_as_u16().ok_or(MISSING_INNER)?.value().into())
            }),
            (ValueUnion::U16Vec, |value| {
                Ok(value
                    .inner_as_u16_vec()
                    .ok_or(MISSING_INNER)?
                    .value()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into())
            }),
            (ValueUnion::OptionalU16, |value| {
                Ok(value
                    .inner_as_optional_u16()
                    .ok_or(MISSING_INNER)?
                    .value()
                    .into())
            }),
            (ValueUnion::I16Vec, |value| {
                Ok(value
                    .inner_as_i16_vec()
                    .ok_or(MISSING_INNER)?
                    .value()
                    .into_iter()
                    .collect::<Vec<_>>()
                    .into())
            }),
            (ValueUnion::I32, |value| {
                Ok(value.inner_as_i32().ok_or(MISSING_INNER)?.value().into())
            }),
            (ValueUnion::String, |value| {
                Ok(Cow::from(
                    value
                        .inner_as_string()
                        .ok_or(MISSING_INNER)?
                        .value()
                        .to_owned(),
                )
                .into())
            }),
            (ValueUnion::StringVec, |value| {
                Ok(value
                    .inner_as_string_vec()
                    .ok_or(MISSING_INNER)?
                    .value()
                    .into_iter()
                    .map(|s| Cow::Owned(s.to_owned()))
                    .collect::<Vec<_>>()
                    .into())
            }),
            (ValueUnion::OptionalString, |value| {
                Ok(value
                    .inner_as_optional_string()
                    .ok_or(MISSING_INNER)?
                    .value()
                    .map(ToOwned::to_owned)
                    .map(Cow::Owned)
                    .into())
            }),
        ];
        // Since ENUM_VALUES uses constants rather than an enum, we can't use match to ensure
        // all variants are covered. static_assertions is used as a workaround.
        static_assertions::const_assert_eq!(
            PARSERS.len(),
            generated::value::ValueUnion::ENUM_VALUES.len(),
        );

        PARSERS
            .iter()
            .find(|(key, _)| *key == value.inner_type())
            .ok_or(FromFlatbufferError::InvalidUnionVariant {
                union_name: "ValueUnion",
                variant: value.inner_type().0,
            })
            .map(|(_, parser)| parser(value))?
    }
}
