import { Stack } from "@mui/material";
import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { EqualizerHelper } from "../../../wasm/pkg/openscq30_web_wasm";
import { Device } from "../../bluetooth/Device";
import { useToastErrorHandler } from "../../hooks/useToastErrorHandler";
import {
  EqualizerConfiguration,
  PresetEqualizerProfile,
  SoundModes,
} from "../../libTypes/DeviceState";
import { DeviceInfo } from "../deviceInfo/DeviceInfo";
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
  device: Device;
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

  const [displayState, setDisplayState] = useDisplayState(
    device,
    onBluetoothError,
  );

  const [isCreateCustomProfileDialogOpen, setCreateCustomProfileDialogOpen] =
    useState(false);
  const customEqualizerProfiles = useCustomEqualizerProfiles();

  const setSelectedPresetProfile = useCallback(
    (presetProfile: PresetEqualizerProfile | "custom") => {
      const newEqualizerConfiguration: EqualizerConfiguration =
        presetProfile != "custom"
          ? {
              presetProfile,
              volumeAdjustments: [
                ...EqualizerHelper.getPresetProfileVolumeAdjustments(
                  presetProfile,
                ),
              ],
            }
          : {
              presetProfile: null,
              volumeAdjustments:
                displayState.equalizerConfiguration.volumeAdjustments,
            };
      setDisplayState((state) => ({
        ...state,
        equalizerConfiguration: newEqualizerConfiguration,
      }));
    },
    [displayState.equalizerConfiguration.volumeAdjustments, setDisplayState],
  );

  const setEqualizerValue = useCallback(
    (changedIndex: number, newVolume: number) => {
      setDisplayState((state) => {
        const volumeAdjustments =
          state.equalizerConfiguration.volumeAdjustments.map((volume, index) =>
            index == changedIndex ? newVolume : volume,
          );
        return {
          ...state,
          equalizerConfiguration: {
            presetProfile: null,
            volumeAdjustments: volumeAdjustments,
          },
        };
      });
    },
    [setDisplayState],
  );

  const setSoundModes = useCallback(
    (soundModes: SoundModes) => {
      setDisplayState((state) => ({
        ...state,
        soundModes: soundModes,
      }));
    },
    [setDisplayState],
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
    displayState.equalizerConfiguration.volumeAdjustments,
  );

  return (
    <Stack spacing={2}>
      {displayState.deviceProfile.soundMode != null &&
        displayState.soundModes && (
          <SoundModeSelection
            soundModes={displayState.soundModes}
            setSoundModes={setSoundModes}
            options={{
              hasTransparencyModes:
                displayState.deviceProfile.soundMode.transparencyModeType ==
                "custom",
              noiseCanceling:
                displayState.deviceProfile.soundMode.noiseCancelingModeType,
            }}
          />
        )}
      {displayState.deviceProfile.numEqualizerBands > 0 && (
        <EqualizerSettings
          profile={
            displayState.equalizerConfiguration.presetProfile ?? "custom"
          }
          onProfileSelected={setSelectedPresetProfile}
          values={displayState.equalizerConfiguration.volumeAdjustments}
          onValueChange={setEqualizerValue}
          customProfiles={customEqualizerProfiles}
          onAddCustomProfile={openCreateCustomProfileDialog}
          onDeleteCustomProfile={deleteCustomProfile}
        />
      )}
      <DeviceInfo deviceState={displayState} />
      <NewCustomProfileDialog
        isOpen={isCreateCustomProfileDialogOpen}
        existingProfiles={customEqualizerProfiles}
        onClose={closeCreateCustomProfileDialog}
        onCreate={createCustomProfileWithName}
      />
    </Stack>
  );
}
