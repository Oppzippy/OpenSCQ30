import { Stack } from "@mui/material";
import {
  EqualizerBandOffsets,
  EqualizerConfiguration,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";
import { AmbientSoundModeSelection } from "./AmbientSoundModeSelection";
import { EqualizerSettings } from "./EqualizerSettings";
import { NoiseCancelingModeSelection } from "./NoiseCancelingModeSelection";
import { SoundcoreDevice } from "../bluetooth/SoundcoreDevice";
import { useBehaviorSubject } from "../hooks/useObservable";

export function DeviceSettings({ device }: { device: SoundcoreDevice }) {
  const state = useBehaviorSubject(device.state);
  const values = [
    ...state.equalizerConfiguration.bandOffsets.volumeOffsets,
  ].map((value) => value / 10);

  async function onEqualizerValueChange(index: number, newValue: number) {
    const newValues = [
      ...state.equalizerConfiguration.bandOffsets.volumeOffsets,
    ];
    newValues[index] = newValue * 10;
    device.equalizerConfiguration = EqualizerConfiguration.fromCustomProfile(
      new EqualizerBandOffsets(new Int8Array(newValues))
    );
  }

  async function onPresetProfileSelected(profile: PresetEqualizerProfile | -1) {
    if (profile == -1) {
      device.equalizerConfiguration = EqualizerConfiguration.fromCustomProfile(
        state.equalizerConfiguration.bandOffsets
      );
    } else {
      device.equalizerConfiguration =
        EqualizerConfiguration.fromPresetProfile(profile);
    }
  }

  return (
    <Stack spacing={2}>
      <AmbientSoundModeSelection
        value={state.ambientSoundMode}
        onValueChanged={(ambientSoundMode) =>
          (device.ambientSoundMode = ambientSoundMode)
        }
      />
      <NoiseCancelingModeSelection
        value={state.noiseCancelingMode}
        onValueChanged={(noiseCancelingMode) =>
          (device.noiseCancelingMode = noiseCancelingMode)
        }
      />
      <EqualizerSettings
        profile={state.equalizerConfiguration.presetProfile ?? -1}
        onProfileSelected={onPresetProfileSelected}
        values={values}
        onValueChange={onEqualizerValueChange}
      />
    </Stack>
  );
}
