import { addTag } from "@/data/commands/database";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { tagKeys } from "../query-keys";

export const useAddTag = () => {
  const queryClient = useQueryClient();

  return useMutation({
    mutationFn: addTag,
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: tagKeys.all(),
      });
    },
  });
};
