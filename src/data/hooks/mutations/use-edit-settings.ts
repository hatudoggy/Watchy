import { useMutation, useQueryClient } from "@tanstack/react-query";
import { settingsKeys } from "../query-keys";
import { setSettings } from "@/data/managers/settings-manager";

export const useEditSettings = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: setSettings,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: settingsKeys.all(),
      });
    },
  });
};
