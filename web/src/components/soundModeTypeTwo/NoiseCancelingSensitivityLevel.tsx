import { Box, Slider, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";

interface Props {
  value: number;
  onValueChanged: (newValue: number) => void;
}

export const NoiseCancelingSensitivityLevel = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();

  return (
    <Box>
      <Typography id="noise-canceling-sensitivity-level-label">
        {t("soundModes.noiseCancelingSensitivityLevel")}
      </Typography>
      <Slider
        value={value}
        min={0}
        max={10}
        step={1}
        valueLabelDisplay="auto"
        aria-labelledby="noise-canceling-sensitivity-level-label"
        onChange={(_, value) => onValueChanged(value as number)}
      />
    </Box>
  );
});
