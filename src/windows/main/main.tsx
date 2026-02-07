import React from "react";
import ReactDOM from "react-dom/client";
import MainWindow from "./main-window";
import { ContextMenuProvider } from "mantine-contextmenu";
import { NuqsAdapter } from "nuqs/adapters/react";
import { QueryClientProvider } from "@tanstack/react-query";
import { ModalsProvider } from "@mantine/modals";
import { modals } from "./modals/modals";
import ThemeLoader from "@/style/theme-loader";
import { warn, debug, trace, info, error } from "@tauri-apps/plugin-log";
import { MantineProvider } from "@mantine/core";
import { theme } from "@/style/mantine-theme";
import { queryClient } from "@/data/query-client";

import "mantine-contextmenu/styles.css";
import "@/global.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <NuqsAdapter>
      <QueryClientProvider client={queryClient}>
        <MantineProvider theme={theme} defaultColorScheme="auto">
          <ThemeLoader />
          <ContextMenuProvider>
            <ModalsProvider modals={modals}>
              <MainWindow />
            </ModalsProvider>
          </ContextMenuProvider>
        </MantineProvider>
      </QueryClientProvider>
    </NuqsAdapter>
  </React.StrictMode>,
);

function forwardConsole(
  fnName: "log" | "debug" | "info" | "warn" | "error",
  logger: (message: string) => Promise<void>,
) {
  const original = console[fnName];
  console[fnName] = (message) => {
    original(message);
    logger(message);
  };
}

forwardConsole("log", trace);
forwardConsole("debug", debug);
forwardConsole("info", info);
forwardConsole("warn", warn);
forwardConsole("error", error);
