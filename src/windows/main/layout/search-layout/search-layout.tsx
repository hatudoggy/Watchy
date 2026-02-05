import IconButton from "@/components/ui/icon-button";
import TextField from "@/components/ui/text-field";
import { useSearchFilter } from "@/data/states/use-search-filter";
import {
  IconFilter,
  IconFilterFilled,
  IconPlus,
  IconSearch,
  IconSettings,
} from "@tabler/icons-react";
import { classifyInput } from "./classify-input";
import VideoPreview from "./video-preview";
import Stack from "@/components/ui/stack";
import Group from "@/components/ui/group";
import { useShowFilters } from "@/data/states/use-show-filters";
import { Indicator, Select } from "@mantine/core";
import { useSortVideosState } from "./hooks/use-sort-videos-state";
import TagsCombobox from "../../components/tags-combobox";
import { useFilterTagsState } from "./hooks/use-filter-tags-state";
import { useTagsFilter } from "@/data/states/use-tags-filter";
import Text from "@/components/ui/text";
import { modalsManager } from "../../modals/modals-manager";

export default function SearchLayout() {
  const [searchFilter, setSearchFilter] = useSearchFilter();
  const [showFilters] = useShowFilters();

  return (
    <Stack className="flex-none gap-0 px-4 pt-2">
      <Group className="gap-2 pt-2">
        {/* Search Field */}
        <TextField
          className="flex-1"
          placeholder="Search or Add..."
          leftSection={
            classifyInput(searchFilter).type === "youtube-url" ? (
              <IconPlus size={20} />
            ) : (
              <IconSearch size={20} />
            )
          }
          value={searchFilter}
          onChange={(e) => setSearchFilter(e.target.value)}
          clearable
          onClear={() => setSearchFilter("")}
        />

        {/* Side Buttons */}
        <Group className="gap-0.5">
          <FilterButton />
          <SettingsButton />
        </Group>
      </Group>

      {/* Video Preview */}
      {classifyInput(searchFilter).type === "youtube-url" && (
        <VideoPreview link={searchFilter} />
      )}

      {classifyInput(searchFilter).type === "empty" && showFilters && (
        <FiltersBar />
      )}
    </Stack>
  );
}

function FilterButton() {
  const [showFilters, setShowFilters] = useShowFilters();
  const [tags] = useTagsFilter();

  return (
    <Indicator
      styles={{
        indicator: {
          paddingInline: 0,
        },
      }}
      classNames={{
        indicator: "border-2 border-black",
      }}
      offset={8}
      size={16}
      disabled={!tags}
      label={<Text size="10">{tags?.length}</Text>}
    >
      <IconButton
        variant="subtle"
        size="input-sm"
        onClick={() => {
          setShowFilters((val) => !val);
        }}
      >
        {showFilters ? (
          <IconFilterFilled size={20} />
        ) : (
          <IconFilter size={20} />
        )}
      </IconButton>
    </Indicator>
  );
}

function FiltersBar() {
  const {
    field,
    direction,
    fieldOptions,
    directionOptions,
    handleFieldChange,
    handleDirectionChange,
  } = useSortVideosState();

  const { allTags, tags, handleAddTag, handleRemoveTag } = useFilterTagsState();

  return (
    <Group className="items-start pt-2">
      <Group className="flex-1">
        <TagsCombobox
          variant="unstyled"
          allTags={allTags || []}
          tags={tags}
          placeholder="Filter Tags"
          onAddTag={(tag) => handleAddTag(tag.id)}
          onRemoveTag={handleRemoveTag}
        />
      </Group>
      <Group gap={1}>
        <Select
          classNames={{
            input:
              "rounded-l-full bg-white/5 border border-white/10 pl-3 hover:bg-white/20",
          }}
          variant="unstyled"
          w={95}
          size="xs"
          checkIconPosition="right"
          withCheckIcon={false}
          rightSection={null}
          allowDeselect={false}
          value={field}
          onChange={handleFieldChange}
          data={fieldOptions}
        />
        <Select
          classNames={{
            input:
              "rounded-r-full bg-white/5 border border-white/10 pl-2 hover:bg-white/20",
          }}
          variant="unstyled"
          w={55}
          radius="xl"
          size="xs"
          withCheckIcon={false}
          rightSection={null}
          allowDeselect={false}
          value={direction}
          onChange={handleDirectionChange}
          data={directionOptions}
        />
      </Group>
    </Group>
  );
}

function SettingsButton() {
  return (
    <IconButton
      variant="subtle"
      size="input-sm"
      onClick={() => modalsManager.settings({})}
    >
      <IconSettings size={20} />
    </IconButton>
  );
}
