import { useAddTag } from "@/data/hooks/mutations/tag-mutation";
import { useEffect } from "react";
import TagsCombobox from "@/windows/main/components/tags-combobox";
import { useVideoPreviewTags } from "./hooks/use-video-preview-tags";
import { Tag } from "@/data/types/database.types";

export default function VideoTags() {
  const { addTagId, removeTagId, reset, allTags, isLoading, tags } =
    useVideoPreviewTags();
  const { mutateAsync } = useAddTag();

  useEffect(() => {
    return () => {
      reset();
    };
  }, []);

  const handleAddTag = (tag: Tag) => addTagId(tag.id);

  const handleCreateTag = async (name: string) => {
    const tag = await mutateAsync({
      tag: { name: name, color: "#000000" },
    });
    addTagId(tag.id);
  };

  const handleRemoveTag = (id: number) => removeTagId(id);

  if (isLoading || !allTags) return null;

  return (
    <TagsCombobox
      variant="unstyled"
      allTags={allTags}
      tags={tags}
      placeholder="Add tags"
      onAddTag={(tag) => handleAddTag(tag)}
      onCreateTag={(tagName) => handleCreateTag(tagName)}
      onRemoveTag={(id) => handleRemoveTag(id)}
    />
  );
}
