import {
  Box,
  Container,
  Toolbar,
  createTheme,
  useMediaQuery,
} from "@mui/material";
import { useCallback, useMemo, useState } from "react";
import { useUpdateAvailableToast } from "../hooks/useUpdateAvailableToast";
import { ConnectedAppBar } from "./ConnectedAppBar";
import { DisconnectedAppBar } from "./DisconnectedAppBar";
import { HomePage } from "./HomePage";
import { DeviceSettings } from "./deviceSettings/DeviceSettings";
import { Device, selectDemoDevice, selectDevice } from "../bluetooth/Device";

export function AppContents() {
  const [device, setDevice] = useState<Device>();
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

  const connect = useCallback(() => {
    (isDemoMode ? selectDemoDevice : selectDevice)()
      .then(setDevice)
      .catch((err) => {
        // Ignore error if the user canceled the device selection popup
        if (!(err instanceof DOMException) || err.name != "NotFoundError") {
          console.error(err);
        }
      });
  }, [isDemoMode]);

  const disconnect = useCallback(() => {
    device?.destroy();
    setDevice(undefined);
  }, [device]);

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
            {device ? (
              <DeviceSettings device={device} disconnect={disconnect} />
            ) : (
              <HomePage />
            )}
          </Container>
        </Box>
      </Box>
    </>
  );
}

export default AppContents;
