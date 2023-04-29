import { Box, Typography } from "@mui/material";
import { NoiseCancelingMode } from "../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";

type Props = {
  value: NoiseCancelingMode;
  onValueChanged: (newValue: NoiseCancelingMode) => void;
};

export function NoiseCancelingModeSelection(props: Props) {
  return (
    <Box>
      <Typography>Ambient Sound Mode</Typography>
      <ToggleButtonRow
        value={props.value}
        onValueChanged={props.onValueChanged}
        values={[
          {
            value: NoiseCancelingMode.Transport,
            displayText: "Transport",
          },
          {
            value: NoiseCancelingMode.Outdoor,
            displayText: "Outdoor",
          },
          {
            value: NoiseCancelingMode.Indoor,
            displayText: "Indoor",
          },
        ]}
      />
    </Box>
  );
}
