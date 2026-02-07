import { useMutation, useQueryClient } from "@tanstack/react-query";
import { addTagToVideo } from "../../commands/database";
import { tagKeys, videoKeys } from "../query-keys";
import { removeTagFromVideo } from "../../commands/database";

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
    meta: {
      successMessage: "Successfully added tag to video",
      errorMessage: "Error adding tag to video",
      log: true,
    },
  });
};

export const useRemoveVideoTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: removeTagFromVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully removed tag from video",
      errorMessage: "Error removing tag from video",
      log: true,
    },
  });
};
