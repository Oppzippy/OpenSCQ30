import { Stack, Typography } from "@mui/material";
import React, { useCallback } from "react";
import { AmbientSoundModeSelection } from "../soundMode/AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "../soundMode/NoiseCancelingModeSelection";
import { SoundModes } from "../../libTypes/DeviceState";
import { TransparencyModeSelection } from "../soundMode/TransparencyModeSelection";
import { CustomNoiseCancelingSelection } from "../soundMode/CustomNoiseCancelingSelection";
import { useTranslation } from "react-i18next";

interface Props {
  soundModes: SoundModes;
  setSoundModes: (soundModes: SoundModes) => void;
  options: {
    hasTransparencyModes: boolean;
    noiseCanceling: "none" | "basic" | "custom";
  };
}

export const SoundModeSelection = React.memo(function ({
  options,
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
      />
      {options.hasTransparencyModes && (
        <TransparencyModeSelection
          value={soundModes.transparencyMode}
          onValueChanged={setTransparencyMode}
        />
      )}
      {options.noiseCanceling != "none" && (
        <NoiseCancelingModeSelection
          value={soundModes.noiseCancelingMode}
          onValueChanged={setNoiseCancelingMode}
          hasCustomMode={options.noiseCanceling == "custom"}
        />
      )}
      {options.noiseCanceling == "custom" &&
        soundModes.noiseCancelingMode == "custom" && (
          <CustomNoiseCancelingSelection
            value={soundModes.customNoiseCanceling}
            onValueChanged={setCustomNoiseCanceling}
          />
        )}
    </Stack>
  );
});
