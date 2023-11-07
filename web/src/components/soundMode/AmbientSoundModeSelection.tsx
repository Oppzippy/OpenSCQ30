import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React, { useCallback } from "react";
import { SoundModes } from "../../libTypes/DeviceState";

interface Props {
  value: SoundModes["ambientSoundMode"];
  hasNoiseCancelingMode: boolean;
  onValueChanged: (newValue: SoundModes["ambientSoundMode"]) => void;
}

export const AmbientSoundModeSelection = React.memo(function ({
  value,
  hasNoiseCancelingMode,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useCallback(
    (newValue: SoundModes["ambientSoundMode"] | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  let values = [
    {
      value: "transparency",
      displayText: t("ambientSoundMode.transparency"),
    },
    {
      value: "normal",
      displayText: t("ambientSoundMode.normal"),
    },
  ];

  if (hasNoiseCancelingMode) {
    values = [
      {
        value: "noiseCanceling",
        displayText: t("ambientSoundMode.noiseCanceling"),
      },
      ...values,
    ];
  }

  return (
    <Box>
      <Typography>{t("soundModes.ambientSoundMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull as (value: string) => void}
        values={values}
      />
    </Box>
  );
});
