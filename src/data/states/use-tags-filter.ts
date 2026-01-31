import { parseAsArrayOf, parseAsInteger, useQueryState } from "nuqs";

export const useTagsFilter = () => {
  return useQueryState("tags-filter", parseAsArrayOf(parseAsInteger));
};
