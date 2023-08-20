import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React, { useCallback } from "react";
import { SoundModes } from "../../libTypes/DeviceState";

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
  const onValueChangedNotNull = useCallback(
    (newValue: SoundModes["noiseCancelingMode"] | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  const values: {
    value: SoundModes["noiseCancelingMode"];
    displayText: string;
  }[] = [
    {
      value: "transport",
      displayText: t("noiseCancelingMode.transport"),
    },
    {
      value: "outdoor",
      displayText: t("noiseCancelingMode.outdoor"),
    },
    {
      value: "indoor",
      displayText: t("noiseCancelingMode.indoor"),
    },
  ];
  if (hasCustomMode) {
    values.push({
      value: "custom",
      displayText: t("noiseCancelingMode.custom"),
    });
  }

  return (
    <Box>
      <Typography>{t("soundModes.noiseCancelingMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull as (value: string) => void}
        values={values}
      />
    </Box>
  );
});
