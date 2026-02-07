import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  channelKeys,
  settingsKeys,
  tagKeys,
  videoKeys,
} from "../../query-keys";
import { importData } from "@/data/commands/settings";
import { MantineColorScheme, useMantineColorScheme } from "@mantine/core";

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
  });
};
