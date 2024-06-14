import { Button } from "@mui/material";
import { useTranslation } from "react-i18next";
import { useToastErrorHandler } from "../../../hooks/useToastErrorHandler";
import { useToasts } from "../../../hooks/useToasts";
import { ExportCustomProfilesState } from "./ExportCustomProfilesState";

export function StateDialogButtons({
  state,
  onStateChange,
}: {
  state: ExportCustomProfilesState;
  onStateChange: (newState: ExportCustomProfilesState) => void;
}) {
  const { t } = useTranslation();
  const { addToast } = useToasts();
  const clipboardErrorHandler = useToastErrorHandler(
    t("errors.failedToCopyToClipboard"),
  );

  if (state.type == "profileSelection") {
    const isAnythingNotSelected = state.selection.some(
      (isSelected) => !isSelected,
    );
    return (
      <Button
        onClick={() => {
          onStateChange({
            ...state,
            selection: new Array(state.selection.length).fill(
              isAnythingNotSelected,
            ),
          });
        }}
      >
        {isAnythingNotSelected
          ? t("application.selectAll")
          : t("application.deselectAll")}
      </Button>
    );
  } else if (state.type == "copyToClipboard") {
    return (
      <Button
        onClick={() => {
          navigator.clipboard
            .writeText(state.profileString)
            .then(() =>
              addToast({ message: t("application.copiedToClipboard") }),
            )
            .catch(clipboardErrorHandler);
        }}
      >
        {t("application.copyToClipboard")}
      </Button>
    );
  }
}
