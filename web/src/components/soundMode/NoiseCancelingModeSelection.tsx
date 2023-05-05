import { Box, Typography } from "@mui/material";
import { NoiseCancelingMode } from "../../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";
import { useTranslation } from "react-i18next";

type Props = {
  value: NoiseCancelingMode;
  onValueChanged: (newValue: NoiseCancelingMode) => void;
};

export function NoiseCancelingModeSelection(props: Props) {
  const { t } = useTranslation();
  return (
    <Box>
      <Typography>{t("noiseCancelingMode.noiseCancelingMode")}</Typography>
      <ToggleButtonRow
        value={props.value}
        onValueChanged={props.onValueChanged}
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
}