import { ThemeProvider, createTheme, useMediaQuery } from "@mui/material";
import { useMemo } from "react";
import { ToastQueue } from "./ToastQueue";
import { AppContents } from "./AppContents";

function App() {
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

  return (
    <ThemeProvider theme={theme}>
      <ToastQueue>
        <AppContents />
      </ToastQueue>
    </ThemeProvider>
  );
}

export default App;
