import { useStatusFilter } from "@/data/states/use-status-filter";
import { VideoStatus } from "@/data/types/database.types";
import { Tabs } from "@mantine/core";
import { IconCheck, IconClock, IconDeviceFloppy } from "@tabler/icons-react";

export default function FilterLayout() {
  const [statusFilter, setStatusFilter] = useStatusFilter();

  return (
    <Tabs
      className="flex-none px-4 pt-2"
      value={statusFilter}
      onChange={(val) => {
        if (val) setStatusFilter(val as VideoStatus);
      }}
    >
      <Tabs.List grow>
        <Tabs.Tab value="SAVED" leftSection={<IconDeviceFloppy size={12} />}>
          Saved
        </Tabs.Tab>
        <Tabs.Tab value="TOWATCH" leftSection={<IconClock size={12} />}>
          To Watch
        </Tabs.Tab>
        <Tabs.Tab value="WATCHED" leftSection={<IconCheck size={12} />}>
          Watched
        </Tabs.Tab>
      </Tabs.List>
    </Tabs>
  );
}
