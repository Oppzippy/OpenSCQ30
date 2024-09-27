import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";
import { SoundModes } from "../../libTypes/DeviceState";
import { useFilterNulls } from "../../hooks/useFilterNulls";

interface Props {
  value: SoundModes["transparencyMode"];
  onValueChanged: (newValue: SoundModes["transparencyMode"]) => void;
}

export const TransparencyModeSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useFilterNulls(onValueChanged);

  return (
    <Box>
      <Typography>{t("soundModes.transparencyMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={
          [
            {
              value: "fullyTransparent",
              label: t("transparencyMode.fullyTransparent"),
            },
            {
              value: "vocalMode",
              label: t("transparencyMode.vocalMode"),
            },
          ] as const
        }
      />
    </Box>
  );
});
