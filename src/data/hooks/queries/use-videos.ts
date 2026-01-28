import { useQuery } from "@tanstack/react-query";
import { videoKeys } from "../query-keys";
import { VideoFilter, VideoSort } from "../../types/database.types";
import { listVideos } from "../../commands/database";

export const useVideos = (filter?: VideoFilter, sort?: VideoSort) => {
  return useQuery({
    queryKey: videoKeys.list(filter, sort),
    queryFn: () =>
      listVideos({
        filter,
        sort,
      }),
    placeholderData: (prev) => prev,
  });
};
