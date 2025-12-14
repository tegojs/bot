import React from "react";
import ReactDOM from "react-dom/client";
import "@excalidraw/excalidraw/index.css";
import { DrawPage } from "../components/Draw/DrawPage";
import "../index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DrawPage />
  </React.StrictMode>,
);
