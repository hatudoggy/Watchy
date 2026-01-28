import IconButton from "@/components/ui/icon-button";
import TextField from "@/components/ui/text-field";
import { useSearchFilter } from "@/data/states/use-search-filter";

import {
  IconFilter,
  IconPlus,
  IconSearch,
  IconSettings,
} from "@tabler/icons-react";
import { classifyInput } from "./classify-input";
import VideoPreview from "./video-preview";
import Stack from "@/components/ui/stack";
import Group from "@/components/ui/group";

export default function SearchLayout() {
  const [searchFilter, setSearchFilter] = useSearchFilter();

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
    </Stack>
  );
}

function FilterButton() {
  return (
    <IconButton variant="subtle" size="input-sm" onClick={() => {}}>
      <IconFilter size={20} />
    </IconButton>
  );
}

function SettingsButton() {
  return (
    <IconButton variant="subtle" size="input-sm" onClick={() => {}}>
      <IconSettings size={20} />
    </IconButton>
  );
}
