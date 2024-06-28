import { Restore, Warning } from "@mui/icons-material";
import {
  Box,
  Checkbox,
  IconButton,
  List,
  ListItem,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  TextField,
  Tooltip,
  Typography,
} from "@mui/material";
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import {
  ImportCustomProfilesState,
  ImportOptionsState,
  getProfileName,
  renameProfile,
  toggleProfileSelection,
} from "./ImportCustomProfilesState";

interface Props {
  state: ImportOptionsState;
  onStateChange: (newState: ImportCustomProfilesState) => void;
}

export function ImportOptions({ state, onStateChange }: Props) {
  const { t } = useTranslation();
  function toggle(index: number) {
    onStateChange(toggleProfileSelection(state, index));
  }

  function rename(index: number, newName: string | undefined) {
    onStateChange(renameProfile(state, index, newName));
  }

  const existingProfileNames = useMemo(
    () => new Set(state.existingProfiles.map((profile) => profile.name)),
    [state.existingProfiles],
  );

  // Jank solution for putting arrays of numbers in a Set by converting them to strings
  const existingProfileValues = useMemo(
    () =>
      new Set(
        state.existingProfiles.map((profile) => JSON.stringify(profile.values)),
      ),
    [state.existingProfiles],
  );

  function warning(index: number, overwrite: boolean) {
    const profile = state.profiles[index];
    const profileName = getProfileName(state, index);
    if (existingProfileValues.has(JSON.stringify(profile.values))) {
      return (
        <Tooltip
          title={
            overwrite
              ? t("equalizer.profileWithSameSettingsAlreadyExists")
              : t("equalizer.profileWithSameSettingsAlreadyExistsNoOverwrite")
          }
        >
          <Warning color={overwrite ? "warning" : "error"} />
        </Tooltip>
      );
    } else if (existingProfileNames.has(profileName)) {
      return (
        <Tooltip title={t("equalizer.profileWithNameAlreadyExists")}>
          <Warning color="warning" />
        </Tooltip>
      );
    }
  }

  return (
    <>
      <Typography>{t("equalizer.importSettings")}</Typography>
      <List>
        <ListItem disablePadding>
          <ListItemButton
            onClick={() =>
              onStateChange({ ...state, overwrite: !state.overwrite })
            }
            dense
          >
            <ListItemIcon>
              <Checkbox
                edge="start"
                checked={state.overwrite}
                disableRipple
                inputProps={{
                  "aria-labelledby": "import-custom-profile-overwrite",
                }}
              />
            </ListItemIcon>
            <ListItemText
              id="import-custom-profile-overwrite"
              primary={t("equalizer.overwriteExistingProfiles")}
            />
          </ListItemButton>
        </ListItem>
      </List>
      <Typography id="import-custom-profile-profiles-header">
        {t("equalizer.profiles")}
      </Typography>
      <TableContainer>
        <Table
          aria-labelledby="import-custom-profile-profiles-header"
          size="small"
        >
          <TableHead>
            <TableRow>
              <TableCell>{t("application.enabled")}</TableCell>
              <TableCell>{t("equalizer.profileName")}</TableCell>
              <TableCell>
                <Tooltip title={t("application.warnings")}>
                  <Warning />
                </Tooltip>
              </TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {state.profiles.map((profile, i) => {
              const labelId = `import-custom-profile-selection-${i}`;
              return (
                <TableRow>
                  <TableCell>
                    <Checkbox
                      checked={state.selection[i]}
                      onClick={() => toggle(i)}
                      aria-labelledby={labelId}
                    />
                  </TableCell>
                  <TableCell>
                    <Box display="flex">
                      <TextField
                        variant="standard"
                        fullWidth
                        value={state.rename[i] ?? profile.name}
                        placeholder={profile.name}
                        onChange={(event) => rename(i, event.target.value)}
                      />
                      <IconButton
                        aria-label={t("equalizer.restoreOriginalName")}
                        sx={{
                          visibility:
                            state.rename[i] != undefined ? "visible" : "hidden",
                        }}
                        onClick={() => rename(i, undefined)}
                      >
                        <Restore />
                      </IconButton>
                    </Box>
                  </TableCell>
                  <TableCell>{warning(i, state.overwrite)}</TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </TableContainer>
    </>
  );
}
