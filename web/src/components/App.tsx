import { ThemeProvider, createTheme } from "@mui/material";
import { ToastQueue } from "./ToastQueue";
import { AppContents } from "./AppContents";

function App() {
  const theme = createTheme({
    colorSchemes: {
      light: true,
      dark: true,
    },
  });
  return (
    <ThemeProvider theme={theme}>
      <ToastQueue>
        <AppContents />
      </ToastQueue>
    </ThemeProvider>
  );
}

export default App;
