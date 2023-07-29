import { Box, Typography } from "@mui/material";
import { AmbientSoundMode } from "../../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React, { useCallback } from "react";

interface Props {
  value: AmbientSoundMode;
  onValueChanged: (newValue: AmbientSoundMode) => void;
}

export const AmbientSoundModeSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useCallback(
    (newValue: AmbientSoundMode | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  return (
    <Box>
      <Typography>{t("ambientSoundMode.ambientSoundMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={[
          {
            value: AmbientSoundMode.NoiseCanceling,
            displayText: t("ambientSoundMode.noiseCanceling"),
          },
          {
            value: AmbientSoundMode.Transparency,
            displayText: t("ambientSoundMode.transparency"),
          },
          {
            value: AmbientSoundMode.Normal,
            displayText: t("ambientSoundMode.normal"),
          },
        ]}
      />
    </Box>
  );
});
