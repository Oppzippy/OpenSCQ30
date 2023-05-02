import React from "react";
import ReactDOM from "react-dom/client";
import App from "./components/App.tsx";
import { CssBaseline } from "@mui/material";

import "./i18n.ts";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <CssBaseline />
    <App />
  </React.StrictMode>
);
