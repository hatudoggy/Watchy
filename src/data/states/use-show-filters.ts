import { parseAsBoolean, useQueryState } from "nuqs";

export const useShowFilters = () => {
  return useQueryState("show-filters", parseAsBoolean);
};
