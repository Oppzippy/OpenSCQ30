import { AppBar, Button, Toolbar, Typography } from "@mui/material";
import { useTranslation } from "react-i18next";

type Props = {
  onSelectDeviceClick: () => Promise<void>;
};

export function DisconnectedAppBar(props: Props) {
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
          {t("deviceSelection")}
        </Typography>
        {navigator.bluetooth && (
          <Button color="inherit" onClick={() => props.onSelectDeviceClick()}>
            Select Device
          </Button>
        )}
      </Toolbar>
    </AppBar>
  );
}
