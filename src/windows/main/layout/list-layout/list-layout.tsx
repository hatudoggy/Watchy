import Text from "@/components/ui/text";
import { IconVideoOff } from "@tabler/icons-react";
import ListItem from "./list-item";
import Stack from "@/components/ui/stack";
import { useFilteredVideoList } from "./hooks/use-filtered-video-list";
import { useListVirtualization } from "./hooks/use-list-virtualization";

export default function ListLayout() {
  const { videos, search } = useFilteredVideoList();

  const { listRef, listVirtualizer } = useListVirtualization(videos.length);

  if (videos.length === 0) return <EmptyState search={search} />;

  return (
    <div
      ref={listRef}
      className="flex-1 overflow-x-hidden overflow-y-scroll py-2 pr-1.5 pl-4"
    >
      <div
        style={{
          height: `${listVirtualizer.getTotalSize()}px`,
          position: "relative",
        }}
      >
        {listVirtualizer.getVirtualItems().map((row) => {
          const item = videos[row.index];
          return (
            <ListItem
              key={item.video.id}
              id={item.video.id}
              link={item.video.link}
              thumbnail={item.video.thumbnail}
              title={item.video.title}
              channel={{
                id: item.video.channelId,
                link: item.video.channelLink,
                name: item.video.channelName,
              }}
              duration={item.video.duration || 0}
              status={item.video.status}
              tags={item.tags}
            />
          );
        })}
      </div>
    </div>
  );
}

function EmptyState({ search }: { search: string }) {
  return (
    <Stack className="h-[50vh] items-center justify-center gap-0 p-8 text-center opacity-50">
      <IconVideoOff size={64} stroke={1.5} />
      <Text size="lg" fw={500} className="mt-4">
        No videos found
      </Text>
      <Text size="sm" c="dimmed">
        {search
          ? "Try adjusting your search or filters"
          : "Add videos to your library to see them here"}
      </Text>
    </Stack>
  );
}
