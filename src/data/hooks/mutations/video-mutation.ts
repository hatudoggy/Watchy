import { useMutation, useQueryClient } from "@tanstack/react-query";
import { addVideo } from "../../commands/database";
import { videoKeys } from "../query-keys";
import { editVideo } from "../../commands/database";
import { deleteVideo } from "../../commands/database";

export const useAddVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully added video",
      errorMessage: "Error adding video",
      log: true,
    },
  });
};

export const useEditVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: editVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully edited video",
      errorMessage: "Error editing video",
      log: true,
    },
  });
};

export const useDeleteVideo = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: deleteVideo,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: videoKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully deleted video",
      errorMessage: "Error deleting video",
      log: true,
    },
  });
};
