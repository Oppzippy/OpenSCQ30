import {
  Stack,
  Table,
  TableBody,
  TableCell,
  TableRow,
  Typography,
} from "@mui/material";
import React from "react";
import { useTranslation } from "react-i18next";
import { DeviceState } from "../../libTypes/DeviceState";

interface Props {
  deviceState: DeviceState;
}

export const DeviceInfo = React.memo(function ({ deviceState }: Props) {
  const { t } = useTranslation();
  return (
    <Stack spacing={2}>
      <Typography component="h2" variant="h6">
        {t("deviceInfo.deviceInfo")}
      </Typography>
      <Table sx={{ tableLayout: "fixed" }}>
        <TableBody>
          <FirmwareVersion deviceState={deviceState} />
          <SerialNumber deviceState={deviceState} />
          <AgeRange deviceState={deviceState} />
          <DeviceProfile deviceState={deviceState} />
        </TableBody>
      </Table>
    </Stack>
  );
});

interface DeviceStateProps {
  deviceState: DeviceState;
}

function FirmwareVersion({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  if (deviceState.firmwareVersion) {
    const firmwareVersion = formatFirmwareVersion(deviceState.firmwareVersion);
    return (
      <TableRow>
        <TableCell>
          <Typography>{t("deviceInfo.firmwareVersion")}</Typography>
        </TableCell>
        <TableCell>
          <Typography>{firmwareVersion}</Typography>
        </TableCell>
      </TableRow>
    );
  }
}

function formatFirmwareVersion(
  firmwareVersion: NonNullable<DeviceState["firmwareVersion"]>,
) {
  return `${firmwareVersion.major
    .toString()
    .padStart(2, "0")}.${firmwareVersion.minor.toString().padStart(2, "0")}`;
}

function SerialNumber({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  if (deviceState.serialNumber) {
    return (
      <TableRow>
        <TableCell>
          <Typography>{t("deviceInfo.serialNumber")}</Typography>
        </TableCell>
        <TableCell>
          <Typography>{deviceState.serialNumber}</Typography>
        </TableCell>
      </TableRow>
    );
  }
}

function AgeRange({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  if (deviceState.ageRange) {
    return (
      <TableRow>
        <TableCell>
          <Typography>{t("deviceInfo.ageRange")}</Typography>
        </TableCell>
        <TableCell>
          <Typography>{deviceState.ageRange}</Typography>
        </TableCell>
      </TableRow>
    );
  }
}

function DeviceProfile({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  return (
    <TableRow>
      <TableCell>
        <Typography>{t("deviceInfo.deviceProfile")}</Typography>
      </TableCell>
      <TableCell>
        <Typography
          component={"div"}
          sx={{ maxWidth: "100%", overflowX: "scroll" }}
        >
          <pre>{JSON.stringify(deviceState.deviceProfile, undefined, 2)}</pre>
        </Typography>
      </TableCell>
    </TableRow>
  );
}
