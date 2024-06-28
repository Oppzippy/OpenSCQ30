import { Box, TextField } from "@mui/material";
import {
  ImportCustomProfilesState,
  StringInputState,
} from "./ImportCustomProfilesState";
import { useTranslation } from "react-i18next";

interface Props {
  state: StringInputState;
  onStateChange: (newState: ImportCustomProfilesState) => void;
}

export function CustomProfileStringInput({ state, onStateChange }: Props) {
  const { t } = useTranslation();
  let errorMessage = undefined;
  if (state.error instanceof Error) {
    errorMessage = state.error.message;
  } else if (state.error != undefined) {
    errorMessage = t("errors.unknownError");
  }
  return (
    <Box component="form" noValidate autoComplete="off">
      <TextField
        label={t("equalizer.customProfilesJSON")}
        content={state.profileString}
        onChange={(e) =>
          onStateChange({ ...state, profileString: e.target.value })
        }
        error={state.error != undefined}
        helperText={errorMessage}
        fullWidth={true}
        margin="normal"
      />
    </Box>
  );
}
