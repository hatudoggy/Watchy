import { parseAsString, useQueryState } from "nuqs";

export const useSearchFilter = () => {
  return useQueryState("search-filter", parseAsString.withDefault(""));
};
