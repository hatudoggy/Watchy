import Text from "@/components/ui/text";
import { useAddVideo } from "@/data/hooks/mutations/video-mutation";
import { useSearchFilter } from "@/data/states/use-search-filter";
import { Button, Loader } from "@mantine/core";
import {
  IconAlertCircle,
  IconCircleCheckFilled,
  IconPlus,
} from "@tabler/icons-react";
import VideoTags from "./video-tags";
import { useVideoPreviewState } from "./hooks/use-video-preview-state";
import { useVideoPreviewTags } from "./hooks/use-video-preview-tags";
import Stack from "@/components/ui/stack";
import Group from "@/components/ui/group";
import VideoThumbnail from "@/windows/main/components/video-thumbnail";
import { YoutubeMetadata } from "@/data/types/youtube-parser.types";

interface VideoPreviewProps {
  link: string;
}

export default function VideoPreview({ link }: VideoPreviewProps) {
  const [_, setSearchFilter] = useSearchFilter();

  const { data, isLoading, isError, error, videoExists } =
    useVideoPreviewState(link);
  const { tags } = useVideoPreviewTags();
  const { mutate } = useAddVideo();

  const handleAddVideo = (data: YoutubeMetadata) => {
    mutate({ metadata: data, tags });
    setSearchFilter("");
  };

  if (!link || videoExists) return null;
  if (isLoading) return <LoadingState />;
  if (isError) return <ErrorState error={error} />;
  if (!data) return null;

  return (
    <Stack className="mt-3 gap-2">
      {/* Video Preview */}
      <Group className="gap-4 rounded-md border border-neutral-700 bg-neutral-800/50 p-3">
        <VideoThumbnail
          src={data.video.thumbnail}
          duration={data.video.duration}
        />
        <Stack className="flex-1 gap-1">
          <Text size="md" fw={500} c="white" className="line-clamp-2">
            {data.video.title}
          </Text>
          <Group className="items-center gap-1.5">
            <Text size="sm" c="dimmed">
              {data.channel.name}
            </Text>
            {data.channel.verified && <IconCircleCheckFilled size={12} />}
          </Group>
          {data.video.views && (
            <Text size="xs" c="dimmed">
              {data.video.views.toLocaleString()} views
            </Text>
          )}
        </Stack>
      </Group>

      {/* Tags Input */}
      <VideoTags />

      {/* Add Button */}
      <Button
        leftSection={<IconPlus size={16} />}
        variant="light"
        size="sm"
        fullWidth
        onClick={() => handleAddVideo(data)}
      >
        Add to Library
      </Button>
    </Stack>
  );
}

function LoadingState() {
  return (
    <Stack className="mt-3 items-center justify-center gap-3 rounded-md border border-neutral-700 bg-neutral-800/50 p-6">
      <Loader size="md" color="indigo" />
      <Text size="sm" c="dimmed">
        Fetching video metadata…
      </Text>
    </Stack>
  );
}

function ErrorState({ error }: { error: Error }) {
  return (
    <Group className="mt-3 items-start gap-3 rounded-md border border-red-500/30 bg-red-500/5 p-4">
      <IconAlertCircle size={24} color="#ef4444" className="shrink-0" />
      <Stack className="flex flex-1 flex-col gap-1">
        <Text c="red" fw={500} size="sm">
          Could not load this video
        </Text>
        <Text size="xs" c="red">
          {error instanceof Error ? error.message : "Failed to load video"}
        </Text>
      </Stack>
    </Group>
  );
}
