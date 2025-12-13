import React from "react";
import ReactDOM from "react-dom/client";
import { DrawPage } from "./pages/draw/page";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DrawPage />
  </React.StrictMode>,
);
