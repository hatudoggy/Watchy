import { useTags } from "@/data/hooks/queries/tag-query";
import { useNewVideoTags } from "@/data/states/use-new-video-tags";

export function useVideoPreviewTags() {
  const [tagIds, setTagIds] = useNewVideoTags();

  const { data: allTags, isLoading } = useTags();
  const tags =
    allTags?.filter((tag) => tagIds.some((id) => id === tag.id)) ?? [];

  const reset = () => setTagIds([]);

  const addTagId = (id: number) =>
    setTagIds((prev) => (prev.includes(id) ? prev : [...prev, id]));

  const removeTagId = (id: number) =>
    setTagIds((prev) => prev.filter((tagId) => tagId !== id));

  return {
    addTagId,
    removeTagId,
    reset,
    allTags,
    isLoading,
    tags,
  };
}
