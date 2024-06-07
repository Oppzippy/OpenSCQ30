import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
} from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { CopyToClipboard } from "./CopyToClipboard";
import { CustomProfileSelection } from "./CustomProfileSelection";
import {
  ExportCustomProfilesState,
  isLastState,
  nextState,
} from "./ExportCustomProfilesState";
import { StateDialogButtons } from "./StateDialogButtons";

interface Props {
  isOpen: boolean;
  onClose: () => void;
  state: ExportCustomProfilesState;
  onStateChange: (newState: ExportCustomProfilesState) => void;
}

export const ExportCustomProfileDialog = React.memo(function (props: Props) {
  const { t } = useTranslation();

  let dialogContent;
  if (props.state.type == "profileSelection") {
    dialogContent = (
      <CustomProfileSelection
        state={props.state}
        onStateChange={(newState) => props.onStateChange(newState)}
      />
    );
  } else if (props.state.type == "copyToClipboard") {
    dialogContent = <CopyToClipboard state={props.state} />;
  }

  const isLast = isLastState(props.state);

  return (
    <Dialog open={props.isOpen} onClose={props.onClose} fullWidth>
      <DialogTitle>{t("equalizer.exportCustomProfiles")}</DialogTitle>
      <DialogContent>{dialogContent}</DialogContent>
      <DialogActions>
        <StateDialogButtons
          state={props.state}
          onStateChange={props.onStateChange}
        />
        <Button
          onClick={() =>
            isLast
              ? props.onClose()
              : props.onStateChange(nextState(props.state))
          }
        >
          {isLast ? t("application.done") : t("application.next")}
        </Button>
      </DialogActions>
    </Dialog>
  );
});
