import { editTag } from "@/data/commands/database";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { tagKeys } from "../query-keys";

export const useEditTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: editTag,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
    },
  });
};
