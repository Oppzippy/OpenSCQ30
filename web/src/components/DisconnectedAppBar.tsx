import { AppBar, Button, Toolbar, Typography } from "@mui/material";

type Props = {
  onSelectDeviceClick: () => Promise<void>;
};

export function DisconnectedAppBar(props: Props) {
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
          Device Selection
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
