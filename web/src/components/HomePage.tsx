import { GitHub } from "@mui/icons-material";
import { Box, Link, Stack, Typography } from "@mui/material";
import { Trans, useTranslation } from "react-i18next";

export function HomePage() {
  const { t } = useTranslation();
  return (
    <Stack textAlign="center" spacing={2}>
      {navigator.bluetooth == undefined && (
        <Typography>{t("application.webBluetoothNotSupported")}</Typography>
      )}
      <Box>
        <Link
          href="https://github.com/oppzippy/OpenSCQ30"
          color="inherit"
          aria-label={t("github").toString()}
        >
          <GitHub />
        </Link>
      </Box>
    </Stack>
  );
}
