import { Stack } from "@mui/material";
import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import {
  EqualizerConfiguration,
  PresetEqualizerProfile,
  VolumeAdjustments,
} from "../../../wasm/pkg/openscq30_web_wasm";
import { SoundcoreDevice } from "../../bluetooth/SoundcoreDevice";
import { SoundModesState } from "../../bluetooth/SoundcoreDeviceState";
import { useToastErrorHandler } from "../../hooks/useToastErrorHandler";
import { EqualizerSettings } from "../equalizer/EqualizerSettings";
import { NewCustomProfileDialog } from "../equalizer/NewCustomProfileDialog";
import { SoundModeSelection } from "./SoundModeSelection";
import { useCreateCustomProfileWithName } from "./hooks/useCreateCustomProfileWithName";
import { useCustomEqualizerProfiles } from "./hooks/useCustomEqualizerProfiles";
import { useDeleteCustomProfile } from "./hooks/useDeleteCustomProfile";
import { useDisplayState } from "./hooks/useDisplayState";

export function DeviceSettings({
  device,
  disconnect,
}: {
  device: SoundcoreDevice;
  disconnect: () => void;
}) {
  const { t } = useTranslation();
  const errorHandler = useToastErrorHandler(t("errors.disconnected"));
  const onBluetoothError = useCallback(
    (err: Error) => {
      errorHandler(err);
      disconnect();
    },
    [errorHandler, disconnect],
  );

  const [deviceState, setDeviceState] = useDisplayState(
    device,
    onBluetoothError,
  );

  const [isCreateCustomProfileDialogOpen, setCreateCustomProfileDialogOpen] =
    useState(false);
  const customEqualizerProfiles = useCustomEqualizerProfiles();

  const setSelectedPresetProfile = useCallback(
    (profile: PresetEqualizerProfile | -1) => {
      const newEqualizerConfiguration =
        profile == -1
          ? EqualizerConfiguration.fromCustomProfile(
              deviceState.equalizerConfiguration.volumeAdjustments,
            )
          : EqualizerConfiguration.fromPresetProfile(profile);
      setDeviceState((state) => ({
        ...state,
        equalizerConfiguration: newEqualizerConfiguration,
      }));
    },
    [deviceState.equalizerConfiguration.volumeAdjustments, setDeviceState],
  );

  const setEqualizerValue = useCallback(
    (index: number, newVolume: number) => {
      setDeviceState((state) => {
        const volume = new Int8Array(
          state.equalizerConfiguration.volumeAdjustments.adjustments,
        );
        // VolumeAdjustments expects integers (-120 to +120), but the state uses decimals (-12.0 to +12.0)
        volume[index] = newVolume * 10;
        const newEqualizerConfiguration =
          EqualizerConfiguration.fromCustomProfile(
            new VolumeAdjustments(volume),
          );
        return {
          ...state,
          equalizerConfiguration: newEqualizerConfiguration,
        };
      });
    },
    [setDeviceState],
  );

  const fractionalEqualizerVolumes = [
    ...deviceState.equalizerConfiguration.volumeAdjustments.adjustments,
  ].map((volume) => volume / 10);

  const setSoundModes = useCallback(
    (soundModes: SoundModesState) => {
      setDeviceState((state) => ({
        ...state,
        soundModes: soundModes,
      }));
    },
    [setDeviceState],
  );

  const openCreateCustomProfileDialog = useCallback(
    () => setCreateCustomProfileDialogOpen(true),
    [],
  );
  const deleteCustomProfile = useDeleteCustomProfile();

  const closeCreateCustomProfileDialog = useCallback(
    () => setCreateCustomProfileDialogOpen(false),
    [setCreateCustomProfileDialogOpen],
  );

  const createCustomProfileWithName = useCreateCustomProfileWithName(
    fractionalEqualizerVolumes,
  );

  return (
    <Stack spacing={2}>
      {deviceState.soundModes && (
        <SoundModeSelection
          soundModes={deviceState.soundModes}
          setSoundModes={setSoundModes}
        />
      )}
      <EqualizerSettings
        profile={deviceState.equalizerConfiguration.presetProfile ?? -1}
        onProfileSelected={setSelectedPresetProfile}
        values={fractionalEqualizerVolumes}
        onValueChange={setEqualizerValue}
        customProfiles={customEqualizerProfiles}
        onAddCustomProfile={openCreateCustomProfileDialog}
        onDeleteCustomProfile={deleteCustomProfile}
      />
      <NewCustomProfileDialog
        isOpen={isCreateCustomProfileDialogOpen}
        existingProfiles={customEqualizerProfiles}
        onClose={closeCreateCustomProfileDialog}
        onCreate={createCustomProfileWithName}
      />
    </Stack>
  );
}
