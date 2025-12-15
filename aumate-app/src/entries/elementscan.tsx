import React from "react";
import ReactDOM from "react-dom/client";
import { ElementScanPage } from "../components/ElementScan/ElementScanPage";
import "../index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ElementScanPage />
  </React.StrictMode>,
);

