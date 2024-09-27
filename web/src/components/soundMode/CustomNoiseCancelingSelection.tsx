import { Box, Slider, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";

interface Props {
  value: number;
  onValueChanged: (newValue: number) => void;
}

export const CustomNoiseCancelingSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();

  return (
    <Box>
      <Typography id="custom-noise-canceling-label">
        {t("soundModes.customNoiseCanceling")}
      </Typography>
      <Slider
        value={value}
        min={0}
        max={10}
        step={1}
        valueLabelDisplay="auto"
        aria-labelledby="custom-noise-canceling-label"
        onChange={(_, value) => onValueChanged(value as number)}
      />
    </Box>
  );
});
