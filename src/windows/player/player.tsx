import React from "react";
import ReactDOM from "react-dom/client";
import PlayerWindow from "./player-window";
import { MantineProvider } from "@mantine/core";
import { ContextMenuProvider } from "mantine-contextmenu";
import { NuqsAdapter } from "nuqs/adapters/react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ModalsProvider } from "@mantine/modals";
import { theme } from "@/style/mantine-theme";

import "mantine-contextmenu/styles.css";
import "@/global.css";

const queryClient = new QueryClient({});

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <NuqsAdapter>
      <QueryClientProvider client={queryClient}>
        <MantineProvider theme={theme} defaultColorScheme="dark">
          <ContextMenuProvider>
            <ModalsProvider>
              <PlayerWindow />
            </ModalsProvider>
          </ContextMenuProvider>
        </MantineProvider>
      </QueryClientProvider>
    </NuqsAdapter>
  </React.StrictMode>,
);
