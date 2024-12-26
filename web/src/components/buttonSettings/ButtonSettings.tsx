import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  Stack,
  Typography,
} from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import {
  ButtonAction,
  ButtonState,
  CustomButtonActions,
} from "../../libTypes/DeviceState";

export const ButtonSettings = React.memo(function ({
  buttonActions,
  setButtonActions,
}: {
  buttonActions: CustomButtonActions;
  setButtonActions: (buttonActions: CustomButtonActions) => void;
}) {
  const { t } = useTranslation();
  const buttons: {
    key: keyof CustomButtonActions;
    label: string;
  }[] = [
    { key: "leftSingleClick", label: t("buttons.leftSingleClick") },
    { key: "leftDoubleClick", label: t("buttons.leftDoubleClick") },
    { key: "leftLongPress", label: t("buttons.leftLongPress") },
    { key: "rightSingleClick", label: t("buttons.rightSingleClick") },
    { key: "rightDoubleClick", label: t("buttons.rightDoubleClick") },
    { key: "rightLongPress", label: t("buttons.rightLongPress") },
  ];

  return (
    <Stack spacing={2}>
      <Typography component="h2" variant="h6">
        {t("buttons.Buttons")}
      </Typography>
      {buttons.map(({ key, label }) => {
        return (
          <ButtonActionSelection
            key={key}
            buttonKey={key}
            label={label}
            action={getButtonAction(buttonActions[key])}
            setAction={(action: ButtonAction | "disabled") => {
              setButtonActions({
                ...buttonActions,
                [key]: setButtonAction(buttonActions[key], action),
              });
            }}
          />
        );
      })}
    </Stack>
  );
});

function getButtonAction(
  button: CustomButtonActions[keyof CustomButtonActions],
): ButtonAction | "disabled" {
  return button.isEnabled ? button.action : "disabled";
}
function setButtonAction(
  button: ButtonState,
  action: ButtonAction | "disabled",
): ButtonState {
  if (action == "disabled") {
    return {
      isEnabled: false,
      action: button.action,
    };
  }
  return {
    isEnabled: true,
    action: action,
  };
}

const ButtonActionSelection = React.memo(function ({
  label,
  buttonKey,
  action,
  setAction,
}: {
  label: string;
  buttonKey: string;
  action: ButtonAction | "disabled";
  setAction: (action: ButtonAction | "disabled") => void;
}) {
  const { t } = useTranslation();

  const options: {
    label: string;
    value: ButtonAction | "disabled";
  }[] = [
    { label: t("buttonActions.disabled"), value: "disabled" },
    { label: t("buttonActions.volumeUp"), value: "volumeUp" },
    { label: t("buttonActions.volumeDown"), value: "volumeDown" },
    { label: t("buttonActions.previousSong"), value: "previousSong" },
    { label: t("buttonActions.nextSong"), value: "nextSong" },
    { label: t("buttonActions.ambientSoundMode"), value: "ambientSoundMode" },
    { label: t("buttonActions.voiceAssistant"), value: "voiceAssistant" },
    { label: t("buttonActions.playPause"), value: "playPause" },
    { label: t("buttonActions.gameMode"), value: "gameMode" },
  ];
  const labelId = `button-settings-${buttonKey}-label`;

  return (
    <FormControl>
      <InputLabel id={labelId}>{label}</InputLabel>
      <Select
        labelId={labelId}
        value={action}
        label={label}
        onChange={(action) => setAction(action.target.value as ButtonAction)}
      >
        {options.map(({ label, value }) => (
          <MenuItem value={value} key={value}>
            {label}
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
});
