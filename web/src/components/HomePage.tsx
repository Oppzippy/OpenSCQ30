import { GitHub } from "@mui/icons-material";
import { Box, Link, Typography } from "@mui/material";
import { Trans, useTranslation } from "react-i18next";

export function HomePage() {
  const { t } = useTranslation();
  return (
    <>
      {navigator.bluetooth == undefined && (
        <Typography>
          <Trans i18nKey={"homepage.webBluetoothNotSupported"}>
            Web Bluetooth is not supported by your browser. See{" "}
            <Link href="https://caniuse.com/web-bluetooth">caniuse</Link> for a
            list of browsers supporting Web Bluetooth.
          </Trans>
        </Typography>
      )}
      <Typography>{t("homepage.earlyDevelopmentNotice")}</Typography>
      <Box sx={{ textAlign: "center" }}>
        <Link href="https://github.com/oppzippy/OpenSCQ30" color="inherit">
          <GitHub />
        </Link>
      </Box>
    </>
  );
}
