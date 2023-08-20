import { Stack } from "@mui/material";
import { useCallback, useState } from "react";
import { useTranslation } from "react-i18next";
import { useToastErrorHandler } from "../../hooks/useToastErrorHandler";
import { EqualizerSettings } from "../equalizer/EqualizerSettings";
import { NewCustomProfileDialog } from "../equalizer/NewCustomProfileDialog";
import { SoundModeSelection } from "./SoundModeSelection";
import { useCreateCustomProfileWithName } from "./hooks/useCreateCustomProfileWithName";
import { useCustomEqualizerProfiles } from "./hooks/useCustomEqualizerProfiles";
import { useDeleteCustomProfile } from "./hooks/useDeleteCustomProfile";
import { useDisplayState } from "./hooks/useDisplayState";
import { Device } from "../../bluetooth/Device";
import {
  EqualizerConfiguration,
  PresetEqualizerProfile,
  SoundModes,
} from "../../libTypes/DeviceState";
import {
  DeviceFeatureFlags,
  EqualizerHelper,
} from "../../../wasm/pkg/openscq30_web_wasm";
import { DeviceInfo } from "../deviceInfo/DeviceInfo";

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
        // VolumeAdjustments expects integers (-120 to +120), but the state uses decimals (-12.0 to +12.0)
        const volumeAdjustments =
          state.equalizerConfiguration.volumeAdjustments.map((volume, index) =>
            index == changedIndex ? newVolume * 10 : volume,
          );
        return {
          ...state,
          equalizerConfiguration: {
            presetProfile: null,
            volumeAdjustments,
          },
        };
      });
    },
    [setDisplayState],
  );

  const fractionalEqualizerVolumes = [
    ...displayState.equalizerConfiguration.volumeAdjustments,
  ].map((volume) => volume / 10);

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
    fractionalEqualizerVolumes,
  );

  let noiseCanceling: Parameters<
    typeof SoundModeSelection
  >[0]["options"]["noiseCanceling"] = "none";
  if (DeviceFeatureFlags.hasCustomNoiseCanceling(displayState.featureFlags)) {
    noiseCanceling = "custom";
  } else if (
    DeviceFeatureFlags.hasNoiseCancelingMode(displayState.featureFlags)
  ) {
    noiseCanceling = "basic";
  }

  return (
    <Stack spacing={2}>
      {displayState.soundModes && (
        <SoundModeSelection
          soundModes={displayState.soundModes}
          setSoundModes={setSoundModes}
          options={{
            hasTransparencyModes: DeviceFeatureFlags.hasTransparencyModes(
              displayState.featureFlags,
            ),
            noiseCanceling,
          }}
        />
      )}
      <EqualizerSettings
        profile={displayState.equalizerConfiguration.presetProfile ?? "custom"}
        onProfileSelected={setSelectedPresetProfile}
        values={fractionalEqualizerVolumes}
        onValueChange={setEqualizerValue}
        customProfiles={customEqualizerProfiles}
        onAddCustomProfile={openCreateCustomProfileDialog}
        onDeleteCustomProfile={deleteCustomProfile}
      />
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
