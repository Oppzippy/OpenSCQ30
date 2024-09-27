import { Box, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { useFilterNulls } from "../../hooks/useFilterNulls";
import { SoundModesTypeTwo } from "../../libTypes/DeviceState";
import { ToggleButtonRow, ToggleButtonValues } from "./ToggleButtonRow";

interface Props {
  value: SoundModesTypeTwo["adaptiveNoiseCanceling"];
  onValueChanged: (
    newValue: SoundModesTypeTwo["adaptiveNoiseCanceling"],
  ) => void;
}

export const AdaptiveNoiseCancelingSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const values: ToggleButtonValues<
    SoundModesTypeTwo["adaptiveNoiseCanceling"]
  > = [
    {
      value: "lowNoise",
      label: t("adaptiveNoiseCanceling.lowNoise"),
    },
    {
      value: "mediumNoise",
      label: t("adaptiveNoiseCanceling.mediumNoise"),
    },
    {
      value: "highNoise",
      label: t("adaptiveNoiseCanceling.highNoise"),
    },
  ];

  return (
    <Box>
      <Typography>{t("soundModes.adaptiveNoiseCanceling")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
