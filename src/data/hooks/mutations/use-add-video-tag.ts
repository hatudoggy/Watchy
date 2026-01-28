import { useMutation, useQueryClient } from "@tanstack/react-query";
import { addTagToVideo } from "../../commands/database";
import { tagKeys, videoKeys } from "../query-keys";

export const useAddVideoTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addTagToVideo,
    onSuccess: (data, variables) => {
      console.log(
        "Tag added successfully:",
        data,
        "to video:",
        variables.videoId,
      );

      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });

      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
    },
    onError: (error) => {
      console.error("Failed to add tag:", error);
    },
  });
};
