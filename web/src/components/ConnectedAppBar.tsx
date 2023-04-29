import { AppBar, Button, Toolbar, Typography } from "@mui/material";

type Props = {
  deviceName: string;
  onDisconnectClick: () => void;
};

export function ConnectedAppBar(props: Props) {
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
        <Button color="inherit" onClick={() => props.onDisconnectClick()}>
          Disconnect
        </Button>
      </Toolbar>
    </AppBar>
  );
}
