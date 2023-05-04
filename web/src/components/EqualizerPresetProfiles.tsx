import { FormControl, InputLabel, MenuItem, Select } from "@mui/material";
import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";
import { useTranslation } from "react-i18next";
import { usePresetEqualizerProfiles } from "../hooks/usePresetEqualizerProfiles";

type Props = {
  profile: PresetEqualizerProfile | -1;
  onProfileSelected: (presetProfile: PresetEqualizerProfile | -1) => void;
};

export function EqualizerPresetProfiles(props: Props) {
  const { t } = useTranslation();
  const presetProfiles = usePresetEqualizerProfiles();
  return (
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
  );
}
