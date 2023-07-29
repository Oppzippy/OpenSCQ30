import { Box, Typography } from "@mui/material";
import { NoiseCancelingMode } from "../../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React, { useCallback } from "react";

interface Props {
  value: NoiseCancelingMode;
  onValueChanged: (newValue: NoiseCancelingMode) => void;
}

export const NoiseCancelingModeSelection = React.memo(function ({
  value,
  onValueChanged,
}: Props) {
  const { t } = useTranslation();
  // Don't allow deselecting the button
  const onValueChangedNotNull = useCallback(
    (newValue: NoiseCancelingMode | undefined) => {
      if (newValue != undefined) {
        onValueChanged(newValue);
      }
    },
    [onValueChanged],
  );

  return (
    <Box>
      <Typography>{t("noiseCancelingMode.noiseCancelingMode")}</Typography>
      <ToggleButtonRow
        value={value}
        onValueChanged={onValueChangedNotNull}
        values={[
          {
            value: NoiseCancelingMode.Transport,
            displayText: t("noiseCancelingMode.transport"),
          },
          {
            value: NoiseCancelingMode.Outdoor,
            displayText: t("noiseCancelingMode.outdoor"),
          },
          {
            value: NoiseCancelingMode.Indoor,
            displayText: t("noiseCancelingMode.indoor"),
          },
        ]}
      />
    </Box>
  );
});
