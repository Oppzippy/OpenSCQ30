import {
  AppBar,
  Box,
  Button,
  Container,
  ThemeProvider,
  Toolbar,
  Typography,
  createTheme,
  useMediaQuery,
} from "@mui/material";
import { useMemo, useState } from "react";
import { SoundcoreDevice, selectDevice } from "./bluetooth/SoundcoreDevice";
import { DeviceSettings } from "./DeviceSettings";

function App() {
  const [device, setDevice] = useState<SoundcoreDevice>();
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode]
  );

  return (
    <ThemeProvider theme={theme}>
      <Box
        sx={{
          display: "flex",
          backgroundColor: theme.palette.background.default,
          minHeight: "100vh",
        }}
      >
        <AppBar position="absolute">
          <Toolbar>
            <Typography
              component="h1"
              variant="h6"
              color="inherit"
              noWrap
              sx={{ flexGrow: 1 }}
            >
              {device == undefined ? "Device Selection" : device.name}
            </Typography>
            {device == undefined ? (
              <Button
                color="inherit"
                onClick={() => selectDevice().then(setDevice)}
              >
                Select Device
              </Button>
            ) : (
              <Button
                color="inherit"
                onClick={() => {
                  device.disconnect();
                  setDevice(undefined);
                }}
              >
                Disconnect
              </Button>
            )}
          </Toolbar>
        </AppBar>
        <Box component="main" sx={{ flexGrow: 1 }}>
          <Toolbar />
          <Container maxWidth="sm" sx={{ my: 2 }}>
            {device ? <DeviceSettings device={device} /> : undefined}
          </Container>
        </Box>
      </Box>
    </ThemeProvider>
  );
}

export default App;
