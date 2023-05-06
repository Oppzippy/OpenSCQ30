import {
  Autocomplete,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  TextField,
} from "@mui/material";
import { useState } from "react";
import { useTranslation } from "react-i18next";
import { CustomEqualizerProfile } from "../../storage/db";
import React from "react";

type Props = {
  isOpen: boolean;
  onClose: () => void;
  onCreate: (name: string) => void;
  existingProfiles: ReadonlyArray<CustomEqualizerProfile>;
};

export const NewCustomProfileDialog = React.memo(function (props: Props) {
  const { t } = useTranslation();
  const [name, setName] = useState("");

  function close() {
    props.onClose();
    setName("");
  }

  function doesCustomProfileWithNameExist(name: string) {
    // The dexie ignoreCase implementation uses String.to(Lower|Upper)Case, so we use that here rather than
    // LocaleCompare or anything else
    // https://www.codeproject.com/Articles/744986/How-to-do-some-magic-with-indexedDB#pre966396
    const lowerCaseName = name.toLowerCase();
    return props.existingProfiles.some(
      (profile) => profile.name.toLowerCase() == lowerCaseName
    );
  }

  return (
    <Dialog open={props.isOpen} onClose={close}>
      <DialogTitle>{t("equalizer.createCustomProfile")}</DialogTitle>
      <DialogContent>
        <Autocomplete
          sx={{ mt: 1 }}
          freeSolo
          disableClearable
          options={props.existingProfiles.map((profile) => profile.name)}
          renderInput={(params) => (
            <TextField {...params} label={t("equalizer.profileName")} />
          )}
          value={name}
          onInputChange={(_event, value) => setName(value)}
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={close}>{t("application.close")}</Button>
        <Button
          onClick={() => {
            props.onCreate(name);
            close();
          }}
        >
          {doesCustomProfileWithNameExist(name)
            ? t("application.overwrite")
            : t("application.create")}
        </Button>
      </DialogActions>
    </Dialog>
  );
});
