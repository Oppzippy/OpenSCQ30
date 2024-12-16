import { Box, Typography } from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { useFilterNulls } from "../../hooks/useFilterNulls";
import { AvailableSoundModes, SoundModes } from "../../libTypes/DeviceState";
import { ToggleButtonRow } from "./ToggleButtonRow";

interface Props {
  value: SoundModes["ambientSoundMode"];
  availableModes: AvailableSoundModes["ambientSoundModes"];
  onValueChanged: (newValue: SoundModes["ambientSoundMode"]) => void;
}

export const AmbientSoundModeSelection = React.memo(function ({
  value,
  onValueChanged,
  availableModes,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  const translations = {
    transparency: t("ambientSoundMode.transparency"),
    normal: t("ambientSoundMode.normal"),
    noiseCanceling: t("ambientSoundMode.noiseCanceling"),
  };

  const values = availableModes.map((mode) => ({
    value: mode,
    label: translations[mode],
  }));

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
