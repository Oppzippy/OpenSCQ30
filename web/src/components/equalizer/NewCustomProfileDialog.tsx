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

type Props = {
  isOpen: boolean;
  onClose: () => void;
  onCreate: (name: string) => void;
};

export function NewCustomProfileDialog(props: Props) {
  const { t } = useTranslation();
  const [name, setName] = useState("");

  function close() {
    props.onClose();
    setName("");
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
          {t("application.create")}
        </Button>
      </DialogActions>
    </Dialog>
  );
}
