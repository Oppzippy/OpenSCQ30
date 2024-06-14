import { Button, Stack, Typography } from "@mui/material";
import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { useCustomEqualizerProfiles } from "../deviceSettings/hooks/useCustomEqualizerProfiles";
import { ExportCustomProfileDialog } from "./exportCustomProfiles/ExportCustomProfilesDialog";
import { createExportCustomProfilesState } from "./exportCustomProfiles/ExportCustomProfilesState";

export const ImportExport = React.memo(function () {
  const { t } = useTranslation();
  const [isCustomProfileOpen, setCustomProfileOpen] = useState(false);

  const customProfiles = useCustomEqualizerProfiles();
  const [exportProfileState, setExportProfileState] = useState(
    createExportCustomProfilesState(customProfiles),
  );
  // State should be reset whenever custom profiles are changed
  useEffect(() => {
    setExportProfileState(createExportCustomProfilesState(customProfiles));
  }, [customProfiles]);

  return (
    <Stack spacing={2}>
      <Typography component="h2" variant="h6">
        {t("application.importExport")}
      </Typography>

      <ExportCustomProfileDialog
        isOpen={isCustomProfileOpen}
        onClose={() => setCustomProfileOpen(false)}
        state={exportProfileState}
        onStateChange={setExportProfileState}
      />
      <Button
        onClick={() => {
          setExportProfileState(
            createExportCustomProfilesState(customProfiles),
          );
          setCustomProfileOpen(true);
        }}
      >
        {t("equalizer.exportCustomProfiles")}
      </Button>
    </Stack>
  );
});
