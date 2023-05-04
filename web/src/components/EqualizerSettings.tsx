import { Add, Delete } from "@mui/icons-material";
import { IconButton, Stack, Typography } from "@mui/material";
import { isEqual } from "lodash-es";
import { useTranslation } from "react-i18next";
import { PresetEqualizerProfile } from "../../wasm/pkg/openscq30_web_wasm";
import { CustomEqualizerProfile } from "../storage/db";
import { Equalizer } from "./Equalizer";
import { EqualizerCustomProfiles } from "./EqualizerCustomProfiles";
import { EqualizerPresetProfiles } from "./EqualizerPresetProfiles";

// TODO -1 is custom, make this more readable
type Props = {
  profile: PresetEqualizerProfile | -1;
  customProfiles: CustomEqualizerProfile[];
  onProfileSelected: (presetProfile: PresetEqualizerProfile | -1) => void;
  values: number[];
  onValueChange: (index: number, newValue: number) => void;
  onAddCustomProfile: () => void;
  onDeleteCustomProfile: (profile: CustomEqualizerProfile) => void;
};

export function EqualizerSettings(props: Props) {
  const { t } = useTranslation();

  const selectedCustomProfile = props.customProfiles.find((customProfile) =>
    isEqual(customProfile.values, props.values)
  );
  return (
    <Stack spacing={2}>
      <Typography>{t("equalizer.equalizer")}</Typography>
      <EqualizerPresetProfiles
        onProfileSelected={props.onProfileSelected}
        profile={props.profile}
      />
      {props.profile == -1 ? (
        <Stack direction="row" spacing={1}>
          <EqualizerCustomProfiles
            sx={{ flexGrow: 1 }}
            profiles={props.customProfiles}
            onProfileSelected={(profile) => {
              profile.values.forEach((value, index) =>
                props.onValueChange(index, value)
              );
            }}
            selectedProfile={selectedCustomProfile}
          />
          {selectedCustomProfile ? (
            <IconButton
              onClick={() => props.onDeleteCustomProfile(selectedCustomProfile)}
              aria-label={t("equalizer.deleteCustomProfile").toString()}
            >
              <Delete />
            </IconButton>
          ) : (
            <IconButton
              onClick={() => props.onAddCustomProfile()}
              aria-label={t("equalizer.createCustomProfile").toString()}
            >
              <Add />
            </IconButton>
          )}
        </Stack>
      ) : undefined}
      <Equalizer
        disabled={props.profile != -1}
        values={props.values}
        onValueChange={props.onValueChange}
      />
    </Stack>
  );
}
