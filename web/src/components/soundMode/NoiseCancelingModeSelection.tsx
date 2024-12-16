import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";
import { AvailableSoundModes, SoundModes } from "../../libTypes/DeviceState";
import { useFilterNulls } from "../../hooks/useFilterNulls";

interface Props {
  value: SoundModes["noiseCancelingMode"];
  onValueChanged: (newValue: SoundModes["noiseCancelingMode"]) => void;
  availableModes: AvailableSoundModes["noiseCancelingModes"];
}

export const NoiseCancelingModeSelection = React.memo(function ({
  value,
  onValueChanged,
  availableModes,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const translations = {
    transport: t("noiseCancelingMode.transport"),
    outdoor: t("noiseCancelingMode.outdoor"),
    indoor: t("noiseCancelingMode.indoor"),
    custom: t("noiseCancelingMode.custom"),
  };
  const values = availableModes.map((mode) => ({
    value: mode,
    label: translations[mode],
  }));

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
