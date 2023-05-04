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
import { useEffect, useMemo, useRef, useState } from "react";
import { debounce } from "lodash-es";
import { useLiveQuery } from "dexie-react-hooks";
import { db } from "../storage/db";
import { EqualizerNewCustomProfileDialog } from "./EqualizerNewCustomProfileDialog";
import { SoundcoreDeviceState } from "../bluetooth/SoundcoreDeviceState";
import { upsertCustomEqualizerProfile } from "../storage/customEqualizerProfiles";

export function DeviceSettings({ device }: { device: SoundcoreDevice }) {
  const actualState = useBehaviorSubject(device.state);
  const [displayState, setDisplayState] = useState(actualState);
  const [isCreateCustomProfileDialogOpen, setCreateCustomProfileDialogOpen] =
    useState(false);
  const customEqualizerProfiles = useLiveQuery(() =>
    db.customEqualizerProfiles.toArray()
  );

  // Synchronizes the displayed state with the actual state of the headphones. They are
  // different because of the equalizer debouncing.
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

  const isInitialRender = useRef(true);

  // Update real state to match displayed
  useEffect(() => {
    if (isInitialRender.current) {
      isInitialRender.current = false;
    } else {
      setActualEqualizerConfiguration(displayState.equalizerConfiguration);
    }
  }, [displayState.equalizerConfiguration, setActualEqualizerConfiguration]);

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
  }

  function onEqualizerValueChange(index: number, newVolume: number) {
    function transformState(state: SoundcoreDeviceState): SoundcoreDeviceState {
      const volume = [
        ...state.equalizerConfiguration.bandOffsets.volumeOffsets,
      ];
      // EqualizerBandOffsets expects integers (-120 to +120), but the state uses decimals (-12.0 to +12.0)
      volume[index] = newVolume * 10;
      const newEqualizerConfiguration =
        EqualizerConfiguration.fromCustomProfile(
          new EqualizerBandOffsets(new Int8Array(volume))
        );
      return {
        ...state,
        equalizerConfiguration: newEqualizerConfiguration,
      };
    }

    setDisplayState(transformState);
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
        customProfiles={customEqualizerProfiles ?? []}
        onAddCustomProfile={() => setCreateCustomProfileDialogOpen(true)}
        onDeleteCustomProfile={(profileToDelete) => {
          if (profileToDelete.id) {
            db.customEqualizerProfiles.delete(profileToDelete.id);
          } else {
            throw Error(
              `tried to delete profile with undefined id: name "${profileToDelete.name}"`
            );
          }
        }}
      />
      <EqualizerNewCustomProfileDialog
        isOpen={isCreateCustomProfileDialogOpen}
        onClose={() => setCreateCustomProfileDialogOpen(false)}
        onCreate={(name) => {
          upsertCustomEqualizerProfile({
            name,
            values: fractionalEqualizerVolumes,
          });
        }}
      />
    </Stack>
  );
}
