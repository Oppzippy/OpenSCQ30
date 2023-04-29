import { GitHub } from "@mui/icons-material";
import { Box, Link, Typography } from "@mui/material";

export function HomePage() {
  return (
    <>
      {navigator.bluetooth == undefined && (
        <Typography>
          Web Bluetooth is not supported by your browser. See{" "}
          <Link href="https://caniuse.com/web-bluetooth">caniuse</Link> for a
          list of browsers supporting Web Bluetooth.
        </Typography>
      )}
      <Typography>
        The web verison of OpenSCQ30 is still in early development. Please use
        the desktop or android version if you encounter any issues.
      </Typography>
      <Box sx={{ textAlign: "center" }}>
        <Link href="https://github.com/oppzippy/OpenSCQ30" color="inherit">
          <GitHub />
        </Link>
      </Box>
    </>
  );
}
