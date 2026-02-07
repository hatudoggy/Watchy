import { useQuery } from "@tanstack/react-query";
import { ytPreviewKeys } from "../query-keys";
import { fetchYoutubeMetadata } from "../../commands/youtube-parser";

export const useYoutubePreview = (
  link: string,
  options?: { enabled?: boolean },
) => {
  return useQuery({
    queryKey: ytPreviewKeys.get(link),
    queryFn: () => fetchYoutubeMetadata({ url: link }),
    enabled: Boolean(link) && (options?.enabled ?? true),
    staleTime: 100 * 60 * 5,
    retry: 1,
    meta: {
      successMessage: `Successfully fetched video preview`,
      errorMessage: `Error fetching video preview`,
      log: true,
    },
  });
};
