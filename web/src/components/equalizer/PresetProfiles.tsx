import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
} from "@mui/material";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { PresetEqualizerProfile } from "../../../wasm/pkg/openscq30_web_wasm";
import { usePresetEqualizerProfiles } from "../../hooks/usePresetEqualizerProfiles";
import { ProfileRow } from "./ProfileRow";

type Props = {
  profile: PresetEqualizerProfile | -1;
  onProfileSelected: (presetProfile: PresetEqualizerProfile | -1) => void;
};

export const PresetProfiles = React.memo(function (props: Props) {
  const { t } = useTranslation();
  const presetProfiles = usePresetEqualizerProfiles();
  const { onProfileSelected } = props;

  const onSelectChange = useCallback(
    (event: SelectChangeEvent<number>) => {
      if (typeof event.target.value == "number") {
        onProfileSelected(event.target.value);
      } else {
        throw Error(
          `value should be a number, but it is instead a ${typeof event.target
            .value}`,
        );
      }
    },
    [onProfileSelected],
  );

  return (
    <FormControl>
      <InputLabel id="equalizer-profile-select-label">
        {t("equalizer.profile")}
      </InputLabel>
      <Select
        labelId="equalizer-profile-select-label"
        label={t("equalizer.profile")}
        value={props.profile}
        onChange={onSelectChange}
      >
        <MenuItem value={-1}>{t("equalizer.custom")}</MenuItem>
        {presetProfiles.map((profile) => (
          <MenuItem value={profile.id} key={profile.id}>
            <ProfileRow
              name={profile.name}
              volumeAdjustments={profile.values}
            />
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
});
