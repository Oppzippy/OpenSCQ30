import {
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

type Props = {
  isOpen: boolean;
  onClose: () => void;
  onCreate: (name: string) => void;
  existingProfiles: ReadonlyArray<CustomEqualizerProfile>;
};

export function NewCustomProfileDialog(props: Props) {
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
        <TextField
          label={t("equalizer.profileName")}
          type="text"
          fullWidth
          variant="standard"
          onChange={(event) => setName(event.target.value)}
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
}
