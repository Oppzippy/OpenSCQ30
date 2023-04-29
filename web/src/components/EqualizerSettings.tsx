import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";
import { Equalizer } from "./Equalizer";
import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import { presetProfiles } from "./EqualizerProfile";

// TODO -1 is custom, make this more readable
type Props = {
  profile: PresetEqualizerProfile | -1;
  onProfileSelected: (presetProfile: PresetEqualizerProfile | -1) => void;
  values: number[];
  onValueChange: (index: number, newValue: number) => void;
};

export function EqualizerSettings(props: Props) {
  return (
    <Stack spacing={2}>
      <Typography>Equalizer</Typography>
      <FormControl>
        <InputLabel id="equalizer-profile-select-label">Profile</InputLabel>
        <Select
          labelId="equalizer-profile-select-label"
          label="Profile"
          value={props.profile}
          onChange={(event) => {
            if (typeof event.target.value == "number") {
              props.onProfileSelected(event.target.value);
            } else {
              throw Error(
                `value should be a number, but it is instead a ${typeof event
                  .target.value}`
              );
            }
          }}
        >
          <MenuItem value={-1}>Custom</MenuItem>
          {presetProfiles.map(({ name, id }) => (
            <MenuItem value={id} key={id}>
              {name}
            </MenuItem>
          ))}
        </Select>
      </FormControl>
      <Equalizer
        disabled={props.profile != -1}
        values={props.values}
        onValueChange={props.onValueChange}
      />
    </Stack>
  );
}
