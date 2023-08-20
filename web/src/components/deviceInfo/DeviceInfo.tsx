import React from "react";
import { DeviceState } from "../../libTypes/DeviceState";
import { useTranslation } from "react-i18next";
import { Table, TableCell, TableRow, Typography } from "@mui/material";

interface Props {
  deviceState: DeviceState;
}

export const DeviceInfo = React.memo(function ({ deviceState }: Props) {
  return (
    <Table>
      <FirmwareVersion deviceState={deviceState} />
      <SerialNumber deviceState={deviceState} />
      <AgeRange deviceState={deviceState} />
      <FeatureFlags deviceState={deviceState} />
    </Table>
  );
});

interface DeviceStateProps {
  deviceState: DeviceState;
}

function FirmwareVersion({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  if (deviceState.leftFirmwareVersion) {
    let firmwareVersion = formatFirmwareVersion(
      deviceState.leftFirmwareVersion,
    );
    if (deviceState.rightFirmwareVersion) {
      firmwareVersion += `, ${formatFirmwareVersion(
        deviceState.rightFirmwareVersion,
      )}`;
    }
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
  firmwareVersion: NonNullable<DeviceState["leftFirmwareVersion"]>,
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

function FeatureFlags({ deviceState }: DeviceStateProps) {
  const { t } = useTranslation();

  if (deviceState.featureFlags) {
    return (
      <TableRow>
        <TableCell>
          <Typography>{t("deviceInfo.featureFlags")}</Typography>
        </TableCell>
        <TableCell>
          <Typography>{deviceState.featureFlags}</Typography>
        </TableCell>
      </TableRow>
    );
  }
}
