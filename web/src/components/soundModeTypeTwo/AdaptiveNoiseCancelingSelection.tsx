import { Box, Slider, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { SoundModesTypeTwo } from "../../libTypes/DeviceState";

interface Props {
  value: SoundModesTypeTwo["adaptiveNoiseCanceling"];
}

export const AdaptiveNoiseCancelingSelection = React.memo(function ({
  value,
}: Props) {
  const { t } = useTranslation();

  const valueLevelMap = {
    lowNoise: {
      level: 0,
      label: t("adaptiveNoiseCanceling.lowNoise"),
    },
    mediumNoise: {
      level: 1,
      label: t("adaptiveNoiseCanceling.mediumNoise"),
    },
    highNoise: {
      level: 2,
      label: t("adaptiveNoiseCanceling.highNoise"),
    },
  } as const;

  return (
    <Box>
      <Typography id="adaptive-noise-canceling-label">
        {t("soundModes.adaptiveNoiseCanceling")}
      </Typography>
      <Slider
        aria-labelledby="adaptive-noise-canceling-label"
        value={valueLevelMap[value].level}
        aria-valuetext={valueLevelMap[value].label}
        min={valueLevelMap.lowNoise.level}
        max={valueLevelMap.highNoise.level}
        disabled={true}
      />
    </Box>
  );
});
