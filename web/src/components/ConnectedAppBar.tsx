import { AppBar, Button, Toolbar, Typography } from "@mui/material";
import { useTranslation } from "react-i18next";
import { ColorSchemeSelect } from "./ColorSchemeSelect";

interface Props {
  deviceName: string;
  onDisconnectClick: () => void;
}

export function ConnectedAppBar(props: Props) {
  const { t } = useTranslation();
  return (
    <AppBar position="absolute">
      <Toolbar>
        <Typography
          component="h1"
          variant="h6"
          color="inherit"
          noWrap
          sx={{ flexGrow: 1 }}
        >
          {props.deviceName}
        </Typography>
        <ColorSchemeSelect />
        <Button color="inherit" onClick={() => props.onDisconnectClick()}>
          {t("device.disconnect")}
        </Button>
      </Toolbar>
    </AppBar>
  );
}
