import {
  Box,
  Container,
  Toolbar,
  createTheme,
  useMediaQuery,
} from "@mui/material";
import { useMemo, useState } from "react";
import { SoundcoreDevice, selectDevice } from "../bluetooth/SoundcoreDevice";
import { ConnectedAppBar } from "./ConnectedAppBar";
import { DeviceSettings } from "./DeviceSettings";
import { DisconnectedAppBar } from "./DisconnectedAppBar";
import { HomePage } from "./HomePage";
import { useUpdateAvailableToast } from "../hooks/useUpdateAvailableToast";

export function AppContents() {
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
  useUpdateAvailableToast();

  async function connect() {
    const device = await selectDevice();
    setDevice(device);
  }

  function disconnect() {
    device?.disconnect();
    setDevice(undefined);
  }

  return (
    <>
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
    </>
  );
}

export default AppContents;
