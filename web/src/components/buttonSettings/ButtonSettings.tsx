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
import { ButtonAction, CustomButtonModel } from "../../libTypes/DeviceState";

export const ButtonSettings = React.memo(function ({
  buttonModel,
  setButtonModel,
}: {
  buttonModel: CustomButtonModel;
  setButtonModel: (buttonModel: CustomButtonModel) => void;
}) {
  const { t } = useTranslation();
  const buttons: {
    key: keyof CustomButtonModel;
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
            action={getButtonAction(buttonModel[key])}
            setAction={(action: ButtonAction) => {
              setButtonModel({
                ...buttonModel,
                [key]: setButtonAction(buttonModel[key], action),
              });
            }}
          />
        );
      })}
    </Stack>
  );
});

function getButtonAction(
  button: CustomButtonModel[keyof CustomButtonModel],
): ButtonAction {
  if ("action" in button) {
    return button.action;
  } else {
    return button.twsConnectedAction;
  }
}
function setButtonAction<
  ActionType extends CustomButtonModel[keyof CustomButtonModel],
>(button: ActionType, action: ButtonAction): ActionType {
  if ("action" in button) {
    return {
      ...button,
      action: action,
    };
  } else {
    return {
      ...button,
      twsConnectedAction: action,
      twsDisconnectedAction: action,
    };
  }
}

const ButtonActionSelection = React.memo(function ({
  label,
  buttonKey,
  action,
  setAction,
}: {
  label: string;
  buttonKey: string;
  action: ButtonAction;
  setAction: (action: ButtonAction) => void;
}) {
  const { t } = useTranslation();

  const options: {
    label: string;
    value: ButtonAction;
  }[] = [
    { label: t("buttonActions.volumeUp"), value: "volumeUp" },
    { label: t("buttonActions.volumeDown"), value: "volumeDown" },
    { label: t("buttonActions.previousSong"), value: "previousSong" },
    { label: t("buttonActions.nextSong"), value: "nextSong" },
    { label: t("buttonActions.ambientSoundMode"), value: "ambientSoundMode" },
    { label: t("buttonActions.voiceAssistant"), value: "voiceAssistant" },
    { label: t("buttonActions.playPause"), value: "playPause" },
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
