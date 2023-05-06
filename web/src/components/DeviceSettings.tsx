import { Stack } from "@mui/material";
import { useLiveQuery } from "dexie-react-hooks";
import { debounce } from "lodash-es";
import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import {
  AmbientSoundMode,
  EqualizerBandOffsets,
  EqualizerConfiguration,
  NoiseCancelingMode,
  PresetEqualizerProfile,
} from "../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDevice } from "../bluetooth/SoundcoreDevice";
import { useBehaviorSubject } from "../hooks/useObservable";
import { upsertCustomEqualizerProfile } from "../storage/customEqualizerProfiles";
import { CustomEqualizerProfile, db } from "../storage/db";
import { EqualizerSettings } from "./equalizer/EqualizerSettings";
import { NewCustomProfileDialog } from "./equalizer/NewCustomProfileDialog";
import { AmbientSoundModeSelection } from "./soundMode/AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "./soundMode/NoiseCancelingModeSelection";

export function DeviceSettings({ device }: { device: SoundcoreDevice }) {
  const actualState = useBehaviorSubject(device.state);
  const [displayState, setDisplayState] = useState(actualState);
  const [isCreateCustomProfileDialogOpen, setCreateCustomProfileDialogOpen] =
    useState(false);
  const customEqualizerProfiles =
    useLiveQuery(() => db.customEqualizerProfiles.toArray()) ?? [];

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

  const onPresetProfileSelected = useCallback(
    (profile: PresetEqualizerProfile | -1) => {
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
    },
    [actualState.equalizerConfiguration.bandOffsets]
  );

  const onEqualizerValueChange = useCallback(
    (index: number, newVolume: number) => {
      setDisplayState((state) => {
        const volume = new Int8Array(
          state.equalizerConfiguration.bandOffsets.volumeOffsets
        );
        // EqualizerBandOffsets expects integers (-120 to +120), but the state uses decimals (-12.0 to +12.0)
        volume[index] = newVolume * 10;
        const newEqualizerConfiguration =
          EqualizerConfiguration.fromCustomProfile(
            new EqualizerBandOffsets(volume)
          );
        return {
          ...state,
          equalizerConfiguration: newEqualizerConfiguration,
        };
      });
    },
    []
  );

  const fractionalEqualizerVolumes = [
    ...displayState.equalizerConfiguration.bandOffsets.volumeOffsets,
  ].map((volume) => volume / 10);

  const onAmbientSoundModeChanged = useCallback(
    (newAmbientSoundMode: AmbientSoundMode) => {
      device.transitionState({
        ...actualState,
        ambientSoundMode: newAmbientSoundMode,
      });
    },
    [device, actualState]
  );

  const onNoiseCancelingModeChanged = useCallback(
    (newNoiseCancelingMode: NoiseCancelingMode) => {
      device.transitionState({
        ...actualState,
        noiseCancelingMode: newNoiseCancelingMode,
      });
    },
    [device, actualState]
  );

  const onAddCustomProfile = useCallback(
    () => setCreateCustomProfileDialogOpen(true),
    []
  );

  const onDeleteCustomProfile = useCallback(
    (profileToDelete: CustomEqualizerProfile) => {
      if (profileToDelete.id) {
        db.customEqualizerProfiles.delete(profileToDelete.id);
      } else {
        throw Error(
          `tried to delete profile with undefined id: name "${profileToDelete.name}"`
        );
      }
    },
    []
  );

  return (
    <Stack spacing={2}>
      <AmbientSoundModeSelection
        value={actualState.ambientSoundMode}
        onValueChanged={onAmbientSoundModeChanged}
      />
      <NoiseCancelingModeSelection
        value={actualState.noiseCancelingMode}
        onValueChanged={onNoiseCancelingModeChanged}
      />
      <EqualizerSettings
        profile={displayState.equalizerConfiguration.presetProfile ?? -1}
        onProfileSelected={onPresetProfileSelected}
        values={fractionalEqualizerVolumes}
        onValueChange={onEqualizerValueChange}
        customProfiles={customEqualizerProfiles}
        onAddCustomProfile={onAddCustomProfile}
        onDeleteCustomProfile={onDeleteCustomProfile}
      />
      <NewCustomProfileDialog
        isOpen={isCreateCustomProfileDialogOpen}
        existingProfiles={customEqualizerProfiles}
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
