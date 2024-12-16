import { Stack, Typography } from "@mui/material";
import React, { useCallback } from "react";
import { AmbientSoundModeSelection } from "../soundMode/AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "../soundMode/NoiseCancelingModeSelection";
import { AvailableSoundModes, SoundModes } from "../../libTypes/DeviceState";
import { TransparencyModeSelection } from "../soundMode/TransparencyModeSelection";
import { CustomNoiseCancelingSelection } from "../soundMode/CustomNoiseCancelingSelection";
import { useTranslation } from "react-i18next";

interface Props {
  soundModes: SoundModes;
  setSoundModes: (soundModes: SoundModes) => void;
  availableModes: AvailableSoundModes;
}

export const SoundModeSelection = React.memo(function ({
  availableModes,
  soundModes,
  setSoundModes,
}: Props) {
  const { t } = useTranslation();
  const setAmbientSoundMode = useCallback(
    (ambientSoundMode: SoundModes["ambientSoundMode"]) => {
      setSoundModes({
        ...soundModes,
        ambientSoundMode,
      });
    },
    [setSoundModes, soundModes],
  );
  const setTransparencyMode = useCallback(
    (transparencyMode: SoundModes["transparencyMode"]) => {
      setSoundModes({
        ...soundModes,
        transparencyMode,
      });
    },
    [setSoundModes, soundModes],
  );
  const setNoiseCancelingMode = useCallback(
    (noiseCancelingMode: SoundModes["noiseCancelingMode"]) => {
      setSoundModes({
        ...soundModes,
        noiseCancelingMode,
      });
    },
    [setSoundModes, soundModes],
  );
  const setCustomNoiseCanceling = useCallback(
    (customNoiseCanceling: number) => {
      setSoundModes({
        ...soundModes,
        customNoiseCanceling,
      });
    },
    [setSoundModes, soundModes],
  );

  return (
    <Stack spacing="2">
      <Typography component="h2" variant="h6">
        {t("soundModes.soundModes")}
      </Typography>
      <AmbientSoundModeSelection
        value={soundModes.ambientSoundMode}
        onValueChanged={setAmbientSoundMode}
        availableModes={availableModes.ambientSoundModes}
      />
      {availableModes.transparencyModes.length != 0 && (
        <TransparencyModeSelection
          value={soundModes.transparencyMode}
          onValueChanged={setTransparencyMode}
          availableModes={availableModes.transparencyModes}
        />
      )}
      {availableModes.noiseCancelingModes.length != 0 && (
        <NoiseCancelingModeSelection
          value={soundModes.noiseCancelingMode}
          onValueChanged={setNoiseCancelingMode}
          availableModes={availableModes.noiseCancelingModes}
        />
      )}
      {availableModes.customNoiseCanceling && (
        <CustomNoiseCancelingSelection
          value={soundModes.customNoiseCanceling}
          onValueChanged={setCustomNoiseCanceling}
        />
      )}
    </Stack>
  );
});
