import { Button } from "@mui/material";
import { useTranslation } from "react-i18next";
import { ImportCustomProfilesState } from "./ImportCustomProfilesState";

export function StateDialogButtons({
  state,
  onStateChange,
}: {
  state: ImportCustomProfilesState;
  onStateChange: (newState: ImportCustomProfilesState) => void;
}) {
  const { t } = useTranslation();

  if (state.type == "importOptions") {
    const isAnythingNotSelected = state.selection.some(
      (isSelected) => !isSelected,
    );
    return (
      <Button
        onClick={() => {
          onStateChange({
            ...state,
            selection: new Array<boolean>(state.selection.length).fill(
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
  }
}
