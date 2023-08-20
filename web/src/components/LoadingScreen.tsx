import { Box, CircularProgress } from "@mui/material";
import React from "react";

export const LoadingScreen = React.memo(function () {
  return (
    <Box display="flex" justifyContent="center">
      <CircularProgress />
    </Box>
  );
});
