import { useMutation, useQueryClient } from "@tanstack/react-query";
import { channelKeys, settingsKeys, tagKeys, videoKeys } from "../query-keys";
import { setSettings } from "@/data/managers/settings-manager";
import { exportData } from "@/data/commands/settings";
import { importData } from "@/data/commands/settings";
import { setDownloadPath } from "@/data/commands/settings";
import { MantineColorScheme, useMantineColorScheme } from "@mantine/core";

export const useEditSettings = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: setSettings,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: settingsKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully updated settings",
      errorMessage: "Error updating settings",
      log: true,
    },
  });
};

export const useExportData = () => {
  return useMutation({
    mutationFn: exportData,
    onSuccess: () => {},
    onError: (e) => {
      console.error(e);
    },
    meta: {
      successMessage: "Successfully exported data",
      errorMessage: "Error exporting data",
      log: true,
    },
  });
};

export const useImportData = () => {
  const queryClient = useQueryClient();
  const { setColorScheme } = useMantineColorScheme({
    keepTransitions: true,
  });

  return useMutation({
    mutationFn: importData,
    onSuccess: (settings) => {
      queryClient.invalidateQueries({
        queryKey: settingsKeys.all(),
      });
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
      queryClient.invalidateQueries({
        queryKey: channelKeys.all(),
      });
      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
      setColorScheme((settings.theme as MantineColorScheme) || "auto");
    },
    onError: (e) => {
      console.error(e);
    },
    meta: {
      successMessage: "Successfully imported data",
      errorMessage: "Error importing data",
      log: true,
    },
  });
};

export const useSetDownloadPath = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: setDownloadPath,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: settingsKeys.all(),
      });
    },
    onError: (e) => {
      console.error(e);
    },
    meta: {
      successMessage: "Successfully updated download path",
      errorMessage: "Error updating download path",
      log: true,
    },
  });
};
