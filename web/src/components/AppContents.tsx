import {
  Box,
  Container,
  Toolbar,
  createTheme,
  useMediaQuery,
} from "@mui/material";
import { useMemo, useState } from "react";
import { selectDemoDevice } from "../bluetooth/DemoSoundcoreDevice";
import { selectDevice } from "../bluetooth/RealSoundcoreDevice";
import { SoundcoreDevice } from "../bluetooth/SoundcoreDevice";
import { useUpdateAvailableToast } from "../hooks/useUpdateAvailableToast";
import { ConnectedAppBar } from "./ConnectedAppBar";
import { DisconnectedAppBar } from "./DisconnectedAppBar";
import { HomePage } from "./HomePage";
import { DeviceSettings } from "./deviceSettings/DeviceSettings";

export function AppContents() {
  const [device, setDevice] = useState<SoundcoreDevice>();
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  // TODO use a hook for isDemoMode
  const isDemoMode = localStorage.getItem("openscq30:demoMode") == "true";
  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode: prefersDarkMode ? "dark" : "light",
        },
      }),
    [prefersDarkMode],
  );
  useUpdateAvailableToast();

  async function connect() {
    try {
      const device = isDemoMode
        ? await selectDemoDevice()
        : await selectDevice();
      setDevice(device);
    } catch (err) {
      // Ignore error if the user canceled the device selection popup
      if (!(err instanceof DOMException) || err.name != "NotFoundError") {
        console.error(err);
      }
    }
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
          <DisconnectedAppBar
            onSelectDeviceClick={() => connect()}
            showSelectDeviceButton={!!navigator.bluetooth || isDemoMode}
          />
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
