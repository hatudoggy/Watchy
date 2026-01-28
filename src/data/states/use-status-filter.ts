import { parseAsStringLiteral, useQueryState } from "nuqs";
import { VideoStatuses } from "../types/database.types";

export const useStatusFilter = () => {
  return useQueryState(
    "status-filter",
    parseAsStringLiteral(VideoStatuses).withDefault("SAVED"),
  );
};
