import { Box, Typography } from "@mui/material";
import { AmbientSoundMode } from "../../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";
import React from "react";

type Props = {
  value: AmbientSoundMode;
  onValueChanged: (newValue: AmbientSoundMode) => void;
};

export const AmbientSoundModeSelection = React.memo(function (props: Props) {
  const { t } = useTranslation();
  return (
    <Box>
      <Typography>{t("ambientSoundMode.ambientSoundMode")}</Typography>
      <ToggleButtonRow
        value={props.value}
        onValueChanged={props.onValueChanged}
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
