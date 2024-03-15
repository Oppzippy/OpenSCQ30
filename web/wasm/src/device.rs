use std::sync::Arc;

use js_sys::Function;
use macaddr::MacAddr6;
use openscq30_lib::{
    api::device::Device as _,
    demo::device::DemoDevice,
    devices::standard::{
        state::DeviceState,
        structures::{CustomButtonModel, EqualizerConfiguration, SoundModes},
    },
    futures::WasmFutures,
    soundcore_device::device::SoundcoreDevice,
};
use tokio::sync::watch;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::BluetoothDevice;

use crate::web_bluetooth_connection::WebBluetoothConnection;

#[wasm_bindgen]
pub struct Device {
    inner: DeviceImplementation,
}

#[wasm_bindgen]
impl Device {
    #[wasm_bindgen]
    pub async fn new(device: BluetoothDevice) -> Result<Device, JsValue> {
        let connection = WebBluetoothConnection::new(device).await?;
        #[allow(clippy::arc_with_non_send_sync)]
        let device = SoundcoreDevice::<_, WasmFutures>::new(Arc::new(connection))
            .await
            .map_err(|err| format!("{err:?}"))?;
        Ok(Self {
            inner: DeviceImplementation::WebBluetooth(device),
        })
    }

    #[wasm_bindgen(js_name = "newDemo")]
    pub async fn new_demo() -> Device {
        let inner_device = DemoDevice::<WasmFutures>::new("Demo Device", MacAddr6::default()).await;
        Self {
            inner: DeviceImplementation::Demo(inner_device),
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

    #[wasm_bindgen(js_name = "setCustomButtonModel")]
    pub async fn set_custom_button_model(
        &self,
        custom_button_model: String,
    ) -> Result<(), JsValue> {
        let custom_button_model: CustomButtonModel =
            serde_json::from_str(&custom_button_model).map_err(|err| format!("{err:?}"))?;
        self.inner
            .set_custom_button_model(custom_button_model)
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
    pub async fn set_state_change_listener(&self, callback: Function) {
        let mut receiver = self.inner.subscribe_to_state_updates().await;
        wasm_bindgen_futures::spawn_local(async move {
            while let Ok(()) = receiver.changed().await {
                let state = receiver.borrow_and_update();
                let json = serde_json::to_string(&*state).unwrap();
                callback
                    .call1(&JsValue::null(), &json.into())
                    .expect("error handling should be done in javascript");
            }
        })
    }
}

// Dynamic dispatch does not work with async functions in traits
enum DeviceImplementation {
    WebBluetooth(SoundcoreDevice<WebBluetoothConnection, WasmFutures>),
    Demo(DemoDevice<WasmFutures>),
}

impl DeviceImplementation {
    pub async fn subscribe_to_state_updates(&self) -> watch::Receiver<DeviceState> {
        match self {
            DeviceImplementation::WebBluetooth(device) => device.subscribe_to_state_updates().await,
            DeviceImplementation::Demo(device) => device.subscribe_to_state_updates().await,
        }
    }

    pub async fn name(&self) -> openscq30_lib::Result<String> {
        match self {
            DeviceImplementation::WebBluetooth(device) => device.name().await,
            DeviceImplementation::Demo(device) => device.name().await,
        }
    }

    pub async fn state(&self) -> DeviceState {
        match self {
            DeviceImplementation::WebBluetooth(device) => device.state().await,
            DeviceImplementation::Demo(device) => device.state().await,
        }
    }

    pub async fn set_sound_modes(&self, sound_modes: SoundModes) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::WebBluetooth(device) => device.set_sound_modes(sound_modes).await,
            DeviceImplementation::Demo(device) => device.set_sound_modes(sound_modes).await,
        }
    }

    pub async fn set_equalizer_configuration(
        &self,
        configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::WebBluetooth(device) => {
                device.set_equalizer_configuration(configuration).await
            }
            DeviceImplementation::Demo(device) => {
                device.set_equalizer_configuration(configuration).await
            }
        }
    }

    pub async fn set_custom_button_model(
        &self,
        custom_button_model: CustomButtonModel,
    ) -> openscq30_lib::Result<()> {
        match self {
            DeviceImplementation::WebBluetooth(device) => {
                device.set_custom_button_model(custom_button_model).await
            }
            DeviceImplementation::Demo(device) => {
                device.set_custom_button_model(custom_button_model).await
            }
        }
    }
}
