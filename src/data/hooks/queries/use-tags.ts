import { useQuery } from "@tanstack/react-query";
import { listTags } from "../../commands/database";
import { tagKeys } from "../query-keys";

export const useTags = () => {
  return useQuery({
    queryKey: tagKeys.all(),
    queryFn: listTags,
  });
};
