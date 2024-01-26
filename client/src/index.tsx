import * as React from "react";
import * as ReactDOM from "react-dom/client";
import { RouterProvider } from "@tanstack/react-router";
import { router } from "./router";
import "./global.css";

const rootElement = document.getElementById("root");
if (!rootElement) throw new Error("No root element");

const root = ReactDOM.createRoot(rootElement);
root.render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
