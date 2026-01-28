import { useMutation, useQueryClient } from "@tanstack/react-query";
import { deleteVideo } from "../../commands/database";
import { videoKeys } from "../query-keys";

export const useDeleteVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
  });
};
