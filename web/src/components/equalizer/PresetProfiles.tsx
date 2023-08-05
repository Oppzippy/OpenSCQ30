import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
} from "@mui/material";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { usePresetEqualizerProfiles } from "../../hooks/usePresetEqualizerProfiles";
import { ProfileRow } from "./ProfileRow";
import { PresetEqualizerProfile } from "../../libTypes/DeviceState";

interface Props {
  profile: PresetEqualizerProfile | "custom";
  onProfileSelected: (presetProfile: PresetEqualizerProfile | "custom") => void;
}

export const PresetProfiles = React.memo(function (props: Props) {
  const { t } = useTranslation();
  const presetProfiles = usePresetEqualizerProfiles();
  const { onProfileSelected } = props;

  const onSelectChange = useCallback(
    (event: SelectChangeEvent<string>) => {
      if (typeof event.target.value == "string") {
        onProfileSelected(event.target.value as PresetEqualizerProfile);
      } else {
        throw Error(
          `value should be a string, but it is instead a ${typeof event.target
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
        <MenuItem value={"custom"}>{t("equalizer.custom")}</MenuItem>
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
