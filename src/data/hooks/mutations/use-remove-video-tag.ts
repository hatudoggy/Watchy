import { useMutation, useQueryClient } from "@tanstack/react-query";
import { removeTagFromVideo } from "../../commands/database";
import { videoKeys } from "../query-keys";

export const useRemoveVideoTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: removeTagFromVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
  });
};
