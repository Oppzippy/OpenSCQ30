import { Add, Delete } from "@mui/icons-material";
import { IconButton, Stack, SxProps, Typography } from "@mui/material";
import { isEqual } from "lodash-es";
import React, { useCallback } from "react";
import { useTranslation } from "react-i18next";
import { PresetEqualizerProfile } from "../../../wasm/pkg/openscq30_web_wasm";
import { CustomEqualizerProfile } from "../../storage/db";
import { CustomProfiles } from "./CustomProfiles";
import { Equalizer } from "./Equalizer";
import { PresetProfiles } from "./PresetProfiles";

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

const customProfilesSx: SxProps = {
  flexGrow: 1,
};

export const EqualizerSettings = React.memo(function (props: Props) {
  const { t } = useTranslation();

  const isCustomProfile = props.profile == -1;

  let selectedCustomProfile: CustomEqualizerProfile | undefined;
  if (isCustomProfile) {
    selectedCustomProfile = props.customProfiles.find((customProfile) =>
      isEqual(customProfile.values, props.values),
    );
  }

  const { onValueChange, onDeleteCustomProfile, onAddCustomProfile } = props;

  const onCustomProfileSelected = useCallback(
    (profile: CustomEqualizerProfile) => {
      profile.values.forEach((value, index) => onValueChange(index, value));
    },
    [onValueChange],
  );

  const deleteSelectedCustomProfile = useCallback(() => {
    if (selectedCustomProfile) {
      onDeleteCustomProfile(selectedCustomProfile);
    }
  }, [onDeleteCustomProfile, selectedCustomProfile]);

  return (
    <Stack spacing={2}>
      <Typography>{t("equalizer.equalizer")}</Typography>
      <PresetProfiles
        onProfileSelected={props.onProfileSelected}
        profile={props.profile}
      />
      <Stack direction="row" spacing={1} alignItems="center">
        <CustomProfiles
          sx={customProfilesSx}
          profiles={props.customProfiles}
          onProfileSelected={onCustomProfileSelected}
          selectedProfile={selectedCustomProfile}
        />
        {/* hide buttons when preset profile is selected */}
        {isCustomProfile &&
          (selectedCustomProfile ? (
            <IconButton
              onClick={deleteSelectedCustomProfile}
              aria-label={t("equalizer.deleteCustomProfile").toString()}
            >
              <Delete />
            </IconButton>
          ) : (
            <IconButton
              onClick={onAddCustomProfile}
              aria-label={t("equalizer.createCustomProfile").toString()}
            >
              <Add />
            </IconButton>
          ))}
      </Stack>
      <Equalizer values={props.values} onValueChange={onValueChange} />
    </Stack>
  );
});
