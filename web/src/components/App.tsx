import {
  Box,
  Container,
  ThemeProvider,
  Toolbar,
  createTheme,
  useMediaQuery,
} from "@mui/material";
import { useMemo, useState } from "react";
import { SoundcoreDevice, selectDevice } from "../bluetooth/SoundcoreDevice";
import { DeviceSettings } from "./DeviceSettings";
import { ConnectedAppBar } from "./ConnectedAppBar";
import { DisconnectedAppBar } from "./DisconnectedAppBar";
import { HomePage } from "./HomePage";

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

  async function connect() {
    const device = await selectDevice();
    setDevice(device);
  }
  function disconnect() {
    device?.disconnect();
    setDevice(undefined);
  }

  return (
    <ThemeProvider theme={theme}>
      <Box
        sx={{
          display: "flex",
          backgroundColor: theme.palette.background.default,
          minHeight: "100vh",
        }}
        color="text.primary"
      >
        {device ? (
          <ConnectedAppBar
            deviceName={device.name ?? "Unknown device"}
            onDisconnectClick={() => disconnect()}
          />
        ) : (
          <DisconnectedAppBar onSelectDeviceClick={() => connect()} />
        )}
        <Box component="main" sx={{ flexGrow: 1 }}>
          <Toolbar />
          <Container maxWidth="sm" sx={{ my: 2 }}>
            {device ? <DeviceSettings device={device} /> : <HomePage />}
          </Container>
        </Box>
      </Box>
    </ThemeProvider>
  );
}

export default App;
