import { Box, Container, Toolbar, useTheme } from "@mui/material";
import { useCallback, useState } from "react";
import { useUpdateAvailableToast } from "../hooks/useUpdateAvailableToast";
import { ConnectedAppBar } from "./ConnectedAppBar";
import { DisconnectedAppBar } from "./DisconnectedAppBar";
import { HomePage } from "./HomePage";
import { DeviceSettings } from "./deviceSettings/DeviceSettings";
import { Device, selectDemoDevice, selectDevice } from "../bluetooth/Device";
import { LoadingScreen } from "./LoadingScreen";

export function AppContents() {
  const [device, setDevice] = useState<Device>();
  const [isLoading, setLoading] = useState(false);
  const isDemoMode = localStorage.getItem("openscq30:demoMode") == "true";
  const theme = useTheme();
  useUpdateAvailableToast();

  const connect = useCallback(
    (isFiltered: boolean) => {
      setLoading(true);
      (isDemoMode ? selectDemoDevice : selectDevice)(isFiltered)
        .then(setDevice)
        .catch((err) => {
          setLoading(false);
          // Ignore error if the user canceled the device selection popup
          if (!(err instanceof DOMException) || err.name != "NotFoundError") {
            console.error(err);
          }
        });
    },
    [isDemoMode],
  );

  const connectFiltered = useCallback(() => {
    connect(true);
  }, [connect]);
  const connectUnfiltered = useCallback(() => {
    connect(false);
  }, [connect]);

  const disconnect = useCallback(() => {
    device?.destroy();
    setDevice(undefined);
    setLoading(false);
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
            onDisconnectClick={disconnect}
          />
        ) : (
          <DisconnectedAppBar
            onSelectDeviceClick={connectFiltered}
            onSelectDeviceUnfilteredClick={connectUnfiltered}
            showSelectDeviceButton={!!navigator.bluetooth || isDemoMode}
          />
        )}
        <Box component="main" sx={{ flexGrow: 1 }}>
          <Toolbar />
          <Container sx={{ my: 2 }}>
            {device ? (
              <DeviceSettings device={device} disconnect={disconnect} />
            ) : isLoading ? (
              <LoadingScreen />
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
