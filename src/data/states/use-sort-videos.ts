import { parseAsStringEnum, useQueryState } from "nuqs";
import { VideoSort } from "../types/database.types";

export const useSortVideos = () => {
  return useQueryState(
    "sort-videos",
    parseAsStringEnum<VideoSort>(Object.values(VideoSort)).withDefault(
      VideoSort.CreatedAtDesc,
    ),
  );
};
