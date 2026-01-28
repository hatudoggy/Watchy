import { useMutation, useQueryClient } from "@tanstack/react-query";
import { addVideo } from "../../commands/database";
import { videoKeys } from "../query-keys";

export const useAddVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
  });
};
