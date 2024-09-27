import { Box, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { useFilterNulls } from "../../hooks/useFilterNulls";
import { SoundModes } from "../../libTypes/DeviceState";
import { ToggleButtonRow, ToggleButtonValues } from "./ToggleButtonRow";

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
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  let values: ToggleButtonValues<SoundModes["ambientSoundMode"]> = [
    {
      value: "transparency",
      label: t("ambientSoundMode.transparency"),
    },
    {
      value: "normal",
      label: t("ambientSoundMode.normal"),
    },
  ];

  if (hasNoiseCancelingMode) {
    values = [
      {
        value: "noiseCanceling",
        label: t("ambientSoundMode.noiseCanceling"),
      },
      ...values,
    ];
  }

  return (
    <Box>
      <Typography>{t("soundModes.ambientSoundMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
