import { useMutation, useQueryClient } from "@tanstack/react-query";
import { editVideo } from "../../commands/database";
import { videoKeys } from "../query-keys";

export const useEditVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: editVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
  });
};
