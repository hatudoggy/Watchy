import { useVideos } from "@/data/hooks/queries/video-query";
import { useSearchFilter } from "@/data/states/use-search-filter";
import { useSortVideos } from "@/data/states/use-sort-videos";
import { useStatusFilter } from "@/data/states/use-status-filter";
import { useTagsFilter } from "@/data/states/use-tags-filter";

export const useFilteredVideoList = () => {
  const [status] = useStatusFilter();
  const [search] = useSearchFilter();
  const [sort] = useSortVideos();
  const [tags] = useTagsFilter();

  const { data } = useVideos(
    {
      status,
      search,
      tagIds: tags || undefined,
    },
    sort,
  );

  const videos = data ?? [];

  return {
    status,
    search,
    sort,
    tags,
    videos,
  };
};
