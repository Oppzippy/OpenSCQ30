import { AppBar, Button, Toolbar, Typography } from "@mui/material";
import { useTranslation } from "react-i18next";
import { ColorSchemeSelect } from "./ColorSchemeSelect";

interface Props {
  onSelectDeviceClick: () => void;
  onSelectDeviceUnfilteredClick: () => void;
  showSelectDeviceButton: boolean;
}

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
          {t("device.deviceSelection")}
        </Typography>
        <ColorSchemeSelect />
        {props.showSelectDeviceButton && (
          <Button color="inherit" onClick={() => props.onSelectDeviceClick()}>
            {t("device.selectDevice")}
          </Button>
        )}
        {props.showSelectDeviceButton && (
          <Button
            color="inherit"
            onClick={() => props.onSelectDeviceUnfilteredClick()}
          >
            {t("device.selectDeviceUnfiltered")}
          </Button>
        )}
      </Toolbar>
    </AppBar>
  );
}
