import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  SxProps,
  Theme,
} from "@mui/material";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { CustomEqualizerProfile } from "../../storage/db";
import { ProfileRow } from "./ProfileRow";

type Props = {
  profiles: CustomEqualizerProfile[];
  selectedProfile: CustomEqualizerProfile | undefined;
  onProfileSelected: (profile: CustomEqualizerProfile) => void;
  sx?: SxProps<Theme>;
};

export const CustomProfiles = React.memo(function ({
  sx,
  profiles,
  selectedProfile,
  onProfileSelected,
}: Props) {
  const { t } = useTranslation();
  const onSelectChange = useCallback(
    (event: SelectChangeEvent<number>) => {
      if (typeof event.target.value == "number") {
        const newProfile = profiles.find(
          (profile) => profile.id == event.target.value,
        );
        if (newProfile) {
          onProfileSelected(newProfile);
        } else {
          throw Error(
            `tried to select custom profile id ${event.target.value}, but it does not exist`,
          );
        }
      } else {
        throw Error(
          `value should be a number, but it is instead a ${typeof event.target
            .value}`,
        );
      }
    },
    [onProfileSelected, profiles],
  );

  return (
    <FormControl sx={sx}>
      <InputLabel id="equalizer-profile-select-label">
        {t("equalizer.customProfile")}
      </InputLabel>
      <Select
        labelId="equalizer-custom-profile-select-label"
        label={t("equalizer.customProfile")}
        value={selectedProfile?.id ?? ""}
        onChange={onSelectChange}
      >
        {profiles.map((profile) => (
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
