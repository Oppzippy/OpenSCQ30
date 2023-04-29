import { Box, Typography } from "@mui/material";
import { AmbientSoundMode } from "../../wasm/pkg/openscq30_web_wasm";
import { ToggleButtonRow } from "./ToggleButtonRow";

type Props = {
  value: AmbientSoundMode;
  onValueChanged: (newValue: AmbientSoundMode) => void;
};

export function AmbientSoundModeSelection(props: Props) {
  return (
    <Box>
      <Typography>Ambient Sound Mode</Typography>
      <ToggleButtonRow
        value={props.value}
        onValueChanged={props.onValueChanged}
        values={[
          {
            value: AmbientSoundMode.NoiseCanceling,
            displayText: "Noise Canceling",
          },
          {
            value: AmbientSoundMode.Transparency,
            displayText: "Transparency",
          },
          {
            value: AmbientSoundMode.Normal,
            displayText: "Normal",
          },
        ]}
      />
    </Box>
  );
}
