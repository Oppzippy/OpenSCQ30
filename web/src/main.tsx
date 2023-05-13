import React from "react";
import ReactDOM from "react-dom/client";
import App from "./components/App.tsx";
import { CssBaseline } from "@mui/material";

import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";
import "./i18n.ts";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <CssBaseline />
    <App />
  </React.StrictMode>,
);
