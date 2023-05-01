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
import { useEffect, useMemo, useState } from "react";
import { debounce } from "lodash";

export function DeviceSettings({ device }: { device: SoundcoreDevice }) {
  const actualState = useBehaviorSubject(device.state);
  const [displayState, setDisplayState] = useState(actualState);
  useEffect(() => {
    // An equalizer configuration change can never be initiated by the headphones, only us,
    // so we don't need to worry about keeping it in sync. Not updating it here fixes a bug
    // where the displayed equalizer state will revert for a short period of time after being
    // changed if a sound mode is changed before the debounce finishes.
    setDisplayState((state) => ({
      ...actualState,
      equalizerConfiguration: state.equalizerConfiguration,
    }));
  }, [actualState]);

  // Debounce equalizer configuration updates since moving the slider will fire events really fast,
  // and we don't want to spam the headphones
  const setActualEqualizerConfiguration = useMemo(() => {
    return debounce(async (config: EqualizerConfiguration) => {
      await device.transitionState({
        ambientSoundMode: device.ambientSoundMode,
        noiseCancelingMode: device.noiseCancelingMode,
        equalizerConfiguration: config,
      });
    }, 500);
  }, [device]);

  function onPresetProfileSelected(profile: PresetEqualizerProfile | -1) {
    const newEqualizerConfiguration =
      profile == -1
        ? EqualizerConfiguration.fromCustomProfile(
            actualState.equalizerConfiguration.bandOffsets
          )
        : EqualizerConfiguration.fromPresetProfile(profile);
    setDisplayState((state) => ({
      ...state,
      equalizerConfiguration: newEqualizerConfiguration,
    }));
    setActualEqualizerConfiguration(newEqualizerConfiguration);
  }

  function onEqualizerValueChange(index: number, newVolume: number) {
    const volume = [
      ...actualState.equalizerConfiguration.bandOffsets.volumeOffsets,
    ];
    // EqualizerBandOffsets expects integers (-120 to +120), but the state uses decimals (-12.0 to +12.0)
    volume[index] = newVolume * 10;

    const newEqualizerConfiguration = EqualizerConfiguration.fromCustomProfile(
      new EqualizerBandOffsets(new Int8Array(volume))
    );
    setDisplayState((state) => ({
      ...state,
      equalizerConfiguration: newEqualizerConfiguration,
    }));
    setActualEqualizerConfiguration(newEqualizerConfiguration);
  }

  const fractionalEqualizerVolumes = [
    ...displayState.equalizerConfiguration.bandOffsets.volumeOffsets,
  ].map((volume) => volume / 10);

  return (
    <Stack spacing={2}>
      <AmbientSoundModeSelection
        value={actualState.ambientSoundMode}
        onValueChanged={(newAmbientSoundMode) => {
          device.transitionState({
            ...actualState,
            ambientSoundMode: newAmbientSoundMode,
          });
        }}
      />
      <NoiseCancelingModeSelection
        value={actualState.noiseCancelingMode}
        onValueChanged={(newNoiseCancelingMode) => {
          device.transitionState({
            ...actualState,
            noiseCancelingMode: newNoiseCancelingMode,
          });
        }}
      />
      <EqualizerSettings
        profile={displayState.equalizerConfiguration.presetProfile ?? -1}
        onProfileSelected={onPresetProfileSelected}
        values={fractionalEqualizerVolumes}
        onValueChange={onEqualizerValueChange}
      />
    </Stack>
  );
}
