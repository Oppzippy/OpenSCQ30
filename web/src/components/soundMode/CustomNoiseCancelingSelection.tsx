import { Box, Slider, Typography } from "@mui/material";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { SoundModes } from "../../libTypes/DeviceState";

interface Props {
  value: number;
  onValueChanged: (newValue: number) => void;
}

export const CustomNoiseCancelingSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useCallback(
    (newValue: SoundModes["customNoiseCanceling"] | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  return (
    <Box>
      <Typography id="custom-noise-canceling-label">
        {t("customNoiseCanceling.customNoiseCanceling")}
      </Typography>
      <Slider
        value={value}
        min={0}
        max={10}
        step={1}
        valueLabelDisplay="auto"
        aria-labelledby="custom-noise-canceling-label"
        onChange={(_, value) => onValueChangedNotNull(value as number)}
      />
    </Box>
  );
});
