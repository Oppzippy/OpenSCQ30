import { Box, Typography } from "@mui/material";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React, { useCallback } from "react";
import { SoundModes } from "../../libTypes/DeviceState";

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
  const onValueChangedNotNull = useCallback(
    (newValue: SoundModes["transparencyMode"] | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  return (
    <Box>
      <Typography>{t("transparencyMode.transparencyMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull as (value: string) => void}
        values={[
          {
            value: "fullyTransparent",
            displayText: t("transparencyMode.fullyTransparent"),
          },
          {
            value: "vocalMode",
            displayText: t("transparencyMode.vocalMode"),
          },
        ]}
      />
    </Box>
  );
});
