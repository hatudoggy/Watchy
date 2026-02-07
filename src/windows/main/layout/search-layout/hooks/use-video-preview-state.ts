import { useVideos } from "@/data/hooks/queries/video-query";
import { useYoutubePreview } from "@/data/hooks/queries/yt-preview-query";

export function useVideoPreviewState(link: string) {
  const { data: videos, isLoading: isVideosLoading } = useVideos({
    search: link,
  });
  const videoExists = !!videos?.length;

  const preview = useYoutubePreview(link, {
    enabled: !!link && !videoExists && !isVideosLoading,
  });

  return {
    videoExists,
    isVideosLoading,
    ...preview,
  };
}
