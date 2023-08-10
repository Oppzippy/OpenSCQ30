use std::rc::Rc;

use js_sys::Function;
use macaddr::MacAddr6;
use openscq30_lib::{
    demo::device::DemoDevice,
    packets::structures::{EqualizerConfiguration, SoundModes},
    q30::device::Q30Device,
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::BluetoothDevice;

use crate::web_bluetooth_connection::WebBluetoothConnection;

#[wasm_bindgen]
pub struct Device {
    inner: Box<dyn openscq30_lib::api::device::Device>,
}

#[wasm_bindgen]
impl Device {
    #[wasm_bindgen]
    pub async fn new(device: BluetoothDevice) -> Result<Device, JsValue> {
        let connection = WebBluetoothConnection::new(device).await?;
        let device = Q30Device::new(Rc::new(connection))
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(Self {
            inner: Box::new(device),
        })
    }

    #[wasm_bindgen(js_name = "newDemo")]
    pub async fn new_demo() -> Device {
        let inner_device = DemoDevice::new("Demo Device", MacAddr6::default()).await;
        Self {
            inner: Box::new(inner_device),
        }
    }

    #[wasm_bindgen(js_name = "getName")]
    pub async fn name(&self) -> Result<String, String> {
        self.inner.name().await.map_err(|err| format!("{err:?}"))
    }

    #[wasm_bindgen(js_name = "setSoundModes")]
    pub async fn set_sound_modes(&self, sound_modes: String) -> Result<(), JsValue> {
        let sound_modes: SoundModes =
            serde_json::from_str(&sound_modes).map_err(|err| format!("{err:?}"))?;
        self.inner
            .set_sound_modes(sound_modes)
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "setEqualizerConfiguration")]
    pub async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: String,
    ) -> Result<(), JsValue> {
        let equalizer_configuration: EqualizerConfiguration =
            serde_json::from_str(&equalizer_configuration).map_err(|err| format!("{err:?}"))?;
        self.inner
            .set_equalizer_configuration(equalizer_configuration)
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "getState")]
    pub async fn state(&self) -> Result<String, JsValue> {
        let state = self.inner.state().await;
        let json = serde_json::to_string(&state).map_err(|err| format!("{err:?}"))?;
        Ok(json)
    }

    #[wasm_bindgen(js_name = "setStateChangeListener")]
    pub fn set_state_change_listener(&self, callback: Function) {
        let mut receiver = self.inner.subscribe_to_state_updates();
        wasm_bindgen_futures::spawn_local(async move {
            loop {
                while let Ok(state) = receiver.recv().await {
                    let json = serde_json::to_string(&state).unwrap();
                    callback
                        .call1(&JsValue::null(), &json.into())
                        .expect("error handling should be done in javascript");
                }
            }
        })
    }
}
