import { ThemeProvider, createTheme } from "@mui/material";
import { ToastQueue } from "./ToastQueue";
import { AppContents } from "./AppContents";

const theme = createTheme({
  colorSchemes: {
    light: true,
    dark: true,
  },
});

function App() {
  return (
    <ThemeProvider theme={theme}>
      <ToastQueue>
        <AppContents />
      </ToastQueue>
    </ThemeProvider>
  );
}

export default App;
