import React from "react";
import ReactDOM from "react-dom/client";
import MainWindow from "./main-window";
import { ContextMenuProvider } from "mantine-contextmenu";
import { NuqsAdapter } from "nuqs/adapters/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ModalsProvider } from "@mantine/modals";
import { modals } from "./modals/modals";
import ThemeLoader from "@/style/theme-loader";

import "mantine-contextmenu/styles.css";
import "@/global.css";
import { MantineProvider } from "@mantine/core";
import { theme } from "@/style/mantine-theme";

const queryClient = new QueryClient({});

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
