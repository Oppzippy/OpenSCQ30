import { Stack, Typography } from "@mui/material";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { SoundModesTypeTwo } from "../../libTypes/DeviceState";
import { TransparencyModeSelection } from "./TransparencyModeSelection";
import { AmbientSoundModeSelection } from "./AmbientSoundModeSelection";
import { NoiseCancelingModeSelection } from "./NoiseCancelingModeSelection";
import { ManualNoiseCancelingSelection } from "./ManualNoiseCancelingSelection";
import { AdaptiveNoiseCancelingSelection } from "./AdaptiveNoiseCancelingSelection";

interface Props {
  soundModes: SoundModesTypeTwo;
  setSoundModes: (soundModes: SoundModesTypeTwo) => void;
}

function useSetter<K extends keyof SoundModesTypeTwo>(
  key: K,
  soundModes: SoundModesTypeTwo,
  setSoundModes: (soundModes: SoundModesTypeTwo) => void,
) {
  return useCallback(
    (value: SoundModesTypeTwo[K]) => {
      setSoundModes({
        ...soundModes,
        [key]: value,
      });
    },
    [key, setSoundModes, soundModes],
  );
}

export const SoundModeSelection = React.memo(function ({
  soundModes,
  setSoundModes,
}: Props) {
  const { t } = useTranslation();
  const setAmbientSoundMode = useSetter(
    "ambientSoundMode",
    soundModes,
    setSoundModes,
  );
  const setTransparencyMode = useSetter(
    "transparencyMode",
    soundModes,
    setSoundModes,
  );
  const setNoiseCancelingMode = useSetter(
    "noiseCancelingMode",
    soundModes,
    setSoundModes,
  );
  const setManualNoiseCanceling = useSetter(
    "manualNoiseCanceling",
    soundModes,
    setSoundModes,
  );

  return (
    <Stack spacing="2">
      <Typography component="h2" variant="h6">
        {t("soundModes.soundModes")}
      </Typography>
      <AmbientSoundModeSelection
        value={soundModes.ambientSoundMode}
        hasNoiseCancelingMode={true}
        onValueChanged={setAmbientSoundMode}
      />
      <TransparencyModeSelection
        value={soundModes.transparencyMode}
        onValueChanged={setTransparencyMode}
      />
      <NoiseCancelingModeSelection
        value={soundModes.noiseCancelingMode}
        onValueChanged={setNoiseCancelingMode}
      />
      {soundModes.noiseCancelingMode == "adaptive" ? (
        <AdaptiveNoiseCancelingSelection
          value={soundModes.adaptiveNoiseCanceling}
        />
      ) : (
        <ManualNoiseCancelingSelection
          value={soundModes.manualNoiseCanceling}
          onValueChanged={setManualNoiseCanceling}
        />
      )}
    </Stack>
  );
});
