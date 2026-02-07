import { useQuery } from "@tanstack/react-query";
import { videoKeys } from "../query-keys";
import { getVideo } from "../../commands/database";
import { VideoFilter, VideoSort } from "../../types/database.types";
import { listVideos } from "../../commands/database";

export const useVideo = (id: number) => {
  return useQuery({
    queryKey: videoKeys.get(id),
    queryFn: () => getVideo({ id }),
    enabled: id != -1,
    meta: {
      successMessage: `Successfully fetched video with id: ${id}`,
      errorMessage: `Error fetching video with id: ${id}`,
      log: true,
    },
  });
};

export const useVideos = (filter?: VideoFilter, sort?: VideoSort) => {
  return useQuery({
    queryKey: videoKeys.list(filter, sort),
    queryFn: () =>
      listVideos({
        filter,
        sort,
      }),
    placeholderData: (prev) => prev,
    meta: {
      successMessage: `Successfully fetched video list`,
      errorMessage: `Error fetching video list`,
      log: true,
    },
  });
};
