import { useTags } from "@/data/hooks/queries/use-tags";
import { useTagsFilter } from "@/data/states/use-tags-filter";

export const useFilterTagsState = () => {
  const { data: allTags } = useTags();

  const [tagFilters, setTagFilters] = useTagsFilter();

  const tags =
    allTags?.filter((tag) => tagFilters?.some((id) => id === tag.id)) ?? [];

  const handleAddTag = (id: number) => {
    setTagFilters((prev) => {
      if (!prev) return [id];
      if (prev.includes(id)) return prev;

      return [id, ...prev];
    });
  };

  const handleRemoveTag = (id: number) => {
    setTagFilters((prev) => {
      if (!prev) return null;

      const next = prev.filter((prevId) => prevId !== id);
      return next.length ? next : null;
    });
  };

  return {
    allTags,
    tags,
    handleAddTag,
    handleRemoveTag,
  };
};
