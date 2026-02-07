import { useMutation } from "@tanstack/react-query";

import { exportData } from "@/data/commands/settings";

export const useExportData = () => {
  return useMutation({
    mutationFn: exportData,
    onSuccess: () => {},
    onError: (e) => {
      console.error(e);
    },
  });
};
