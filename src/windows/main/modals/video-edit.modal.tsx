import Stack from "@/components/ui/stack";
import { Center, SegmentedControl, Title } from "@mantine/core";
import { ContextModalProps } from "@mantine/modals";
import {
  IconCheck,
  IconClock,
  IconDeviceFloppy,
  IconX,
} from "@tabler/icons-react";
import Text from "@/components/ui/text";
import TagsCombobox from "../components/tags-combobox";
import { useTags } from "@/data/hooks/queries/use-tags";
import { useVideo } from "@/data/hooks/queries/use-video";
import { useEditVideo } from "@/data/hooks/mutations/use-edit-video";
import { useAddVideoTag } from "@/data/hooks/mutations/use-add-video-tag";
import { useRemoveVideoTag } from "@/data/hooks/mutations/use-remove-video-tag";
import { VideoStatus } from "@/data/types/database.types";
import VideoItemSimple from "../components/video-item-simple";
import Group from "@/components/ui/group";
import IconButton from "@/components/ui/icon-button";

export interface VideoEditModalProps {
  videoId: number;
}

export default function VideoEditModal({
  id,
  context,
  innerProps,
}: ContextModalProps<VideoEditModalProps>) {
  const { data: videoWithTags, isLoading: isVideoLoading } = useVideo(
    innerProps.videoId,
  );
  const { data: allTags } = useTags();

  const { mutate: editVideo } = useEditVideo();
  const { mutate: addTag } = useAddVideoTag();
  const { mutate: removeTag } = useRemoveVideoTag();

  if (isVideoLoading || !videoWithTags) return null;

  const { video, tags: videoTags } = videoWithTags;

  const currentTags =
    allTags?.filter((tag) =>
      videoTags.some((vidTag) => vidTag.id === tag.id),
    ) ?? [];

  const handleStatusChange = (value: string) => {
    editVideo({
      id: video.id,
      video: { status: value as VideoStatus },
    });
  };

  const handleAddTag = (tag: { name: string }) => {
    addTag({ videoId: video.id, name: tag.name });
  };

  const handleRemoveTag = (tagId: number) => {
    removeTag({ videoId: video.id, tagId });
  };

  const videoStatusOptions = [
    {
      value: "SAVED",
      label: (
        <Center style={{ gap: 8 }}>
          <IconDeviceFloppy size={16} />
          <Text span size="sm" fw={500}>
            Saved
          </Text>
        </Center>
      ),
    },
    {
      value: "TOWATCH",
      label: (
        <Center style={{ gap: 8 }}>
          <IconClock size={16} />
          <Text span size="sm" fw={500}>
            To Watch
          </Text>
        </Center>
      ),
    },
    {
      value: "WATCHED",
      label: (
        <Center style={{ gap: 8 }}>
          <IconCheck size={16} />
          <Text span size="sm" fw={500}>
            Watched
          </Text>
        </Center>
      ),
    },
  ];

  return (
    <Stack className="overflow-hidden px-2 pt-2" gap="md">
      <Stack gap={2}>
        <Group justify="space-between">
          <Title order={2}>Edit Video</Title>
          <IconButton
            radius="xl"
            variant="subtle"
            color="white"
            onClick={() => context.closeModal(id)}
          >
            <IconX />
          </IconButton>
        </Group>

        {/* <Text
          size="sm"
          c="white"
          fw={600}
          style={{ textTransform: "uppercase", letterSpacing: "0.5px" }}
        >
          Edit Video
        </Text> */}
        <VideoItemSimple
          thumbnail={video.thumbnail}
          title={video.title}
          channel={{
            id: video.channelId,
            link: video.channelLink,
            name: video.channelName,
          }}
          duration={video.duration || 0}
          status={video.status}
        />
      </Stack>

      <Stack gap={4}>
        <Text size="sm" c="white" fw={500}>
          Status
        </Text>
        <SegmentedControl
          fullWidth
          radius="md"
          size="md"
          withItemsBorders={false}
          color="blue"
          value={video.status}
          onChange={handleStatusChange}
          data={videoStatusOptions}
          classNames={{
            root: "bg-white/10 border border-white/10",
            indicator: "bg-blue-600 shadow-lg",
            label: "text-stone-300 hover:text-white transition-colors",
          }}
        />
      </Stack>

      <Stack gap={4}>
        <Text size="sm" c="white" fw={500}>
          Tags
        </Text>
        <Stack className="justify-center rounded-md border border-white/10 bg-white/10 px-2">
          <TagsCombobox
            variant="unstyled"
            allTags={allTags || []}
            tags={currentTags}
            onAddTag={(tag) => handleAddTag(tag)}
            onCreateTag={(name) => handleAddTag({ name })}
            onRemoveTag={handleRemoveTag}
          />
        </Stack>
      </Stack>
    </Stack>
  );
}
