import React from "react";
import ReactDOM from "react-dom/client";
import { ScreenshotMode } from "../components/Screenshot/ScreenshotMode";
import "../index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ScreenshotMode />
  </React.StrictMode>,
);
