import { Button, Stack, Typography } from "@mui/material";
import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useToastErrorHandler } from "../../hooks/useToastErrorHandler";
import { useToasts } from "../../hooks/useToasts";
import {
  insertCustomEqualizerProfilesRenameDuplicates,
  upsertCustomEqualizerProfiles,
} from "../../storage/customEqualizerProfiles";
import { useCustomEqualizerProfiles } from "../deviceSettings/hooks/useCustomEqualizerProfiles";
import { ExportCustomProfileDialog } from "./exportCustomProfiles/ExportCustomProfilesDialog";
import { createExportCustomProfilesState } from "./exportCustomProfiles/ExportCustomProfilesState";
import { ImportCustomProfileDialog } from "./importCustomProfiles/ImportCustomProfilesDialog";
import { createImportCustomProfilesState } from "./importCustomProfiles/ImportCustomProfilesState";

export const ImportExport = React.memo(function () {
  const { t } = useTranslation();
  const [isExportCustomProfilesOpen, setExportCustomProfilesOpen] =
    useState(false);
  const [isImportCustomProfilesOpen, setImportCustomProfilesOpen] =
    useState(false);

  const customProfiles = useCustomEqualizerProfiles();
  const [exportProfileState, setExportProfileState] = useState(
    createExportCustomProfilesState(customProfiles),
  );
  const [importProfileState, setImportProfileState] = useState(
    createImportCustomProfilesState(customProfiles),
  );
  // State should be reset whenever custom profiles are changed
  useEffect(() => {
    setExportProfileState(createExportCustomProfilesState(customProfiles));
    setImportProfileState(createImportCustomProfilesState(customProfiles));
  }, [customProfiles]);

  const importFailedHandler = useToastErrorHandler(t("errors.importFailed"));
  const { addToast } = useToasts();
  const importSuccessfulTranslation = t("application.importSuccessful");

  return (
    <Stack spacing={2}>
      <Typography component="h2" variant="h6">
        {t("application.importExport")}
      </Typography>

      <ExportCustomProfileDialog
        isOpen={isExportCustomProfilesOpen}
        onClose={() => setExportCustomProfilesOpen(false)}
        state={exportProfileState}
        onStateChange={setExportProfileState}
      />
      <ImportCustomProfileDialog
        isOpen={isImportCustomProfilesOpen}
        onClose={() => setImportCustomProfilesOpen(false)}
        state={importProfileState}
        onStateChange={setImportProfileState}
        importProfiles={(command) => {
          if (command.overwrite) {
            upsertCustomEqualizerProfiles(command.profiles)
              .then(() => addToast({ message: importSuccessfulTranslation }))
              .catch(importFailedHandler);
          } else {
            insertCustomEqualizerProfilesRenameDuplicates(command.profiles)
              .then(() => addToast({ message: importSuccessfulTranslation }))
              .catch(importFailedHandler);
          }
        }}
      />
      <Button
        onClick={() => {
          setImportProfileState(
            createImportCustomProfilesState(customProfiles),
          );
          setImportCustomProfilesOpen(true);
        }}
      >
        {t("equalizer.importCustomProfiles")}
      </Button>
      <Button
        onClick={() => {
          setExportProfileState(
            createExportCustomProfilesState(customProfiles),
          );
          setExportCustomProfilesOpen(true);
        }}
      >
        {t("equalizer.exportCustomProfiles")}
      </Button>
    </Stack>
  );
});
