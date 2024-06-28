import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
} from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import {
  CustomProfileImportCommand,
  ImportCustomProfilesState,
  isLastState,
  nextState,
  prepareProfilesForImport,
} from "./ImportCustomProfilesState";
import { ImportOptions } from "./ImportOptions";
import { StateDialogButtons } from "./StateDialogButtons";
import { CustomProfileStringInput } from "./StringInput";

interface Props {
  isOpen: boolean;
  onClose: () => void;
  state: ImportCustomProfilesState;
  onStateChange: (newState: ImportCustomProfilesState) => void;
  importProfiles: (profiles: CustomProfileImportCommand) => void;
}

export const ImportCustomProfileDialog = React.memo(function (props: Props) {
  const { t } = useTranslation();

  let dialogContent;
  if (props.state.type == "stringInput") {
    dialogContent = (
      <CustomProfileStringInput
        state={props.state}
        onStateChange={(newState) => props.onStateChange(newState)}
      />
    );
  } else if (props.state.type == "importOptions") {
    dialogContent = (
      <ImportOptions
        state={props.state}
        onStateChange={(newState) => props.onStateChange(newState)}
      />
    );
  }

  return (
    <Dialog open={props.isOpen} onClose={props.onClose} fullWidth>
      <DialogTitle>{t("equalizer.importCustomProfiles")}</DialogTitle>
      <DialogContent>{dialogContent}</DialogContent>
      <DialogActions>
        <StateDialogButtons
          state={props.state}
          onStateChange={props.onStateChange}
        />
        <Button
          onClick={() => {
            if (isLastState(props.state)) {
              props.onClose();
              props.importProfiles(prepareProfilesForImport(props.state));
            } else {
              props.onStateChange(nextState(props.state));
            }
          }}
        >
          {isLastState(props.state)
            ? t("application.import")
            : t("application.next")}
        </Button>
      </DialogActions>
    </Dialog>
  );
});
