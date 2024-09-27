import { Box, Typography } from "@mui/material";
import { ToggleButtonRow, ToggleButtonValues } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";
import { SoundModes } from "../../libTypes/DeviceState";
import { useFilterNulls } from "../../hooks/useFilterNulls";

interface Props {
  value: SoundModes["noiseCancelingMode"];
  onValueChanged: (newValue: SoundModes["noiseCancelingMode"]) => void;
  hasCustomMode: boolean;
}

export const NoiseCancelingModeSelection = React.memo(function ({
  hasCustomMode,
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const values: ToggleButtonValues<SoundModes["noiseCancelingMode"]> = [
    {
      value: "transport",
      label: t("noiseCancelingMode.transport"),
    },
    {
      value: "outdoor",
      label: t("noiseCancelingMode.outdoor"),
    },
    {
      value: "indoor",
      label: t("noiseCancelingMode.indoor"),
    },
  ];
  if (hasCustomMode) {
    values.push({
      value: "custom",
      label: t("noiseCancelingMode.custom"),
    });
  }

  return (
    <Box>
      <Typography>{t("soundModes.noiseCancelingMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
