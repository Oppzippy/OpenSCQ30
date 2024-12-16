import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";
import { AvailableSoundModes, SoundModes } from "../../libTypes/DeviceState";
import { useFilterNulls } from "../../hooks/useFilterNulls";

interface Props {
  value: SoundModes["transparencyMode"];
  onValueChanged: (newValue: SoundModes["transparencyMode"]) => void;
  availableModes: AvailableSoundModes["transparencyModes"];
}

export const TransparencyModeSelection = React.memo(function ({
  value,
  onValueChanged,
  availableModes,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const translations = {
    fullyTransparent: t("transparencyMode.fullyTransparent"),
    vocalMode: t("transparencyMode.vocalMode"),
  };
  const values = availableModes.map((mode) => ({
    value: mode,
    label: translations[mode],
  }));

  return (
    <Box>
      <Typography>{t("soundModes.transparencyMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={values}
      />
    </Box>
  );
});
