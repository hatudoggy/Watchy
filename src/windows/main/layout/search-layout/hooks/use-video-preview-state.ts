import { useVideos } from "@/data/hooks/queries/use-videos";
import { useYoutubePreview } from "@/data/hooks/queries/use-youtube-preview";

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
