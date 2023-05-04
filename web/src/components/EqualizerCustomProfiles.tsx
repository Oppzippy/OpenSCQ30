import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SxProps,
  Theme,
} from "@mui/material";
import { useTranslation } from "react-i18next";
import { CustomEqualizerProfile } from "../storage/db";

type Props = {
  profiles: CustomEqualizerProfile[];
  selectedProfile: CustomEqualizerProfile | undefined;
  onProfileSelected: (profile: CustomEqualizerProfile) => void;
  sx?: SxProps<Theme>;
};

export function EqualizerCustomProfiles({
  sx,
  profiles,
  selectedProfile,
  onProfileSelected,
}: Props) {
  const { t } = useTranslation();
  return (
    <FormControl sx={sx}>
      <InputLabel id="equalizer-profile-select-label">
        {t("equalizer.customProfile")}
      </InputLabel>
      <Select
        labelId="equalizer-custom-profile-select-label"
        label={t("equalizer.customProfile")}
        value={selectedProfile?.id ?? ""}
        onChange={(event) => {
          if (typeof event.target.value == "number") {
            const newProfile = profiles.find(
              (profile) => profile.id == event.target.value
            );
            if (newProfile) {
              onProfileSelected(newProfile);
            } else {
              throw Error(
                `tried to select custom profile id ${event.target.value}, but it does not exist`
              );
            }
          } else {
            throw Error(
              `value should be a number, but it is instead a ${typeof event
                .target.value}`
            );
          }
        }}
      >
        {profiles.map(({ name, id }) => (
          <MenuItem value={id} key={id}>
            {name}
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
}
