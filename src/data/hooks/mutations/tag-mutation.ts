import { addTag } from "@/data/commands/database";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { tagKeys } from "../query-keys";
import { editTag } from "@/data/commands/database";

export const useAddTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addTag,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully added tag",
      errorMessage: "Error adding tag",
      log: true,
    },
  });
};

export const useEditTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: editTag,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
    },
    meta: {
      successMessage: "Successfully edited tag",
      errorMessage: "Error editing tag",
      log: true,
    },
  });
};
