use async_trait::async_trait;
use openscq30_lib_has::Has;
use palette::{Hsv, IntoColor, Srgb, rgb::Rgb};
use strum::IntoEnumIterator;

use crate::{
    api::settings::{Setting, SettingId, Value},
    devices::soundcore::{
        a3876::{
            self,
            structures::{
                AutoLightsOffDurationInMinutes, ColorfulLights, ColorfulLightsBrightness,
            },
        },
        common::settings_manager::{SettingHandler, SettingHandlerResult},
    },
    settings,
};

use super::ColorfulLightsSetting;

#[derive(Default)]
pub struct ColorfulLightsSettingHandler;

#[async_trait]
impl<T> SettingHandler<T> for ColorfulLightsSettingHandler
where
    T: Has<ColorfulLights> + Send,
{
    fn settings(&self) -> Vec<SettingId> {
        ColorfulLightsSetting::iter().map(Into::into).collect()
    }

    fn get(&self, state: &T, setting_id: &SettingId) -> Option<Setting> {
        let colorful_lights: &ColorfulLights = state.get();
        let colorful_lights_setting: ColorfulLightsSetting = (*setting_id).try_into().ok()?;
        match colorful_lights_setting {
            ColorfulLightsSetting::ColorfulLightsEnabled => Some(Setting::Toggle {
                value: colorful_lights.is_enabled,
            }),
            ColorfulLightsSetting::ColorfulLightsBrightness => Some(Setting::I32Range {
                setting: settings::Range {
                    range: 1..=10,
                    step: 1,
                },
                value: colorful_lights.brightness.inner().into(),
            }),
            ColorfulLightsSetting::AutoLightsOffMinutes => Some(Setting::I32Range {
                setting: settings::Range {
                    range: 1..=120,
                    step: 1,
                },
                value: colorful_lights.auto_lights_off_duration.inner().into(),
            }),
            ColorfulLightsSetting::ColorfulLightsColor => {
                let rgb = Srgb::new(
                    f32::from(colorful_lights.color.red) / 255.0,
                    f32::from(colorful_lights.color.green) / 255.0,
                    f32::from(colorful_lights.color.blue) / 255.0,
                );
                let hsv: Hsv = rgb.into_color();

                Some(Setting::HueColorPicker {
                    hue: hsv.hue.into_positive_degrees(),
                })
            }
            ColorfulLightsSetting::ColorfulLightsMode => {
                Some(Setting::select_from_enum_all_variants(colorful_lights.mode))
            }
        }
    }

    async fn set(
        &self,
        state: &mut T,
        setting_id: &SettingId,
        value: Value,
    ) -> SettingHandlerResult<()> {
        let colorful_lights_setting: ColorfulLightsSetting = (*setting_id)
            .try_into()
            .expect("already filtered to valid values only by SettingsManager");
        match colorful_lights_setting {
            ColorfulLightsSetting::ColorfulLightsEnabled => {
                let colorful_lights = state.get_mut();
                colorful_lights.is_enabled = value.try_as_bool()?;
            }
            ColorfulLightsSetting::ColorfulLightsBrightness => {
                let colorful_lights = state.get_mut();
                colorful_lights.brightness = ColorfulLightsBrightness::new(
                    u8::try_from(value.try_as_i32()?.clamp(1, 10))
                        .expect("1-10 is in range for u8"),
                );
            }
            ColorfulLightsSetting::AutoLightsOffMinutes => {
                let colorful_lights = state.get_mut();
                colorful_lights.auto_lights_off_duration = AutoLightsOffDurationInMinutes::new(
                    u8::try_from(value.try_as_i32()?.clamp(1, 120))
                        .expect("1-120 is in range for u8"),
                );
            }
            ColorfulLightsSetting::ColorfulLightsColor => {
                let colorful_lights = state.get_mut();
                let hue = value.try_as_f32()?;
                let rgb_f32: Rgb = Hsv::new(hue, 1.0, 1.0).into_color();
                let rgb: Rgb<_, u8> = rgb_f32.into_format();
                colorful_lights.color = a3876::structures::RgbColor {
                    red: rgb.red,
                    green: rgb.green,
                    blue: rgb.blue,
                };
            }
            ColorfulLightsSetting::ColorfulLightsMode => {
                let colorful_lights = state.get_mut();
                colorful_lights.mode = value.try_as_enum_variant()?;
            }
        }
        Ok(())
    }
}
