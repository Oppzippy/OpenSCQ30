import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import { useTranslation } from "react-i18next";
import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";
import { usePresetEqualizerProfiles } from "../hooks/usePresetEqualizerProfiles";
import { Equalizer } from "./Equalizer";

// TODO -1 is custom, make this more readable
type Props = {
  profile: PresetEqualizerProfile | -1;
  onProfileSelected: (presetProfile: PresetEqualizerProfile | -1) => void;
  values: number[];
  onValueChange: (index: number, newValue: number) => void;
};

export function EqualizerSettings(props: Props) {
  const { t } = useTranslation();
  const presetProfiles = usePresetEqualizerProfiles();
  return (
    <Stack spacing={2}>
      <Typography>{t("equalizer.equalizer")}</Typography>
      <FormControl>
        <InputLabel id="equalizer-profile-select-label">
          {t("equalizer.profile")}
        </InputLabel>
        <Select
          labelId="equalizer-profile-select-label"
          label={t("equalizer.profile")}
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
          <MenuItem value={-1}>{t("equalizer.custom")}</MenuItem>
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
