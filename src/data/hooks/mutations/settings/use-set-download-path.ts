import { useMutation, useQueryClient } from "@tanstack/react-query";
import { settingsKeys } from "../../query-keys";
import { setDownloadPath } from "@/data/commands/settings";

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
  });
};
