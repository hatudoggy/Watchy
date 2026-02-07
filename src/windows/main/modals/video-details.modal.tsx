import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";
import Text from "@/components/ui/text";
import { useChannel } from "@/data/hooks/queries/channel-query";
import { useVideo } from "@/data/hooks/queries/video-query";
import {
  Avatar,
  Button,
  CopyButton,
  Divider,
  SimpleGrid,
  Title,
} from "@mantine/core";
import { ContextModalProps } from "@mantine/modals";
import {
  IconCalendar,
  IconCheck,
  IconCopy,
  IconExternalLink,
  IconEye,
  IconTag,
  IconThumbUp,
  IconX,
} from "@tabler/icons-react";
import VideoThumbnail from "../components/video-thumbnail";
import VideoItemTags from "../components/video-item-tags";
import { openUrl } from "@tauri-apps/plugin-opener";
import IconButton from "@/components/ui/icon-button";

export interface VideoDetailsModalProps {
  videoId: number;
}

export default function VideoDetailsModal({
  id,
  innerProps,
  context,
}: ContextModalProps<VideoDetailsModalProps>) {
  const { data } = useVideo(innerProps.videoId);
  const video = data?.video;
  const { data: channel } = useChannel(video?.channelId || -1);

  if (!data || !video) return null;

  const { tags } = data;

  return (
    <Stack className="p-4" gap="lg">
      <Group justify="space-between">
        <Title order={2}>Video Details</Title>
        <IconButton
          radius="xl"
          variant="subtle"
          color="white"
          onClick={() => context.closeModal(id)}
        >
          <IconX />
        </IconButton>
      </Group>
      <Stack className="mx-auto w-9/12">
        <VideoThumbnail
          src={video.thumbnail}
          h="100%"
          duration={video.duration || 0}
        />
      </Stack>

      <Stack gap="xs">
        <Text c="white" size="xl" fw={700} style={{ lineHeight: 1.2 }}>
          {video.title}
        </Text>
        <Group align="center" gap="sm">
          {channel ? (
            <Group gap="xs">
              <Avatar src={channel.avatar} size="sm" radius="xl" />
              <Text c="white" size="sm" fw={500}>
                {channel.name}
              </Text>
            </Group>
          ) : (
            <Text size="sm" c="dimmed">
              {video.channelName}
            </Text>
          )}

          <Text size="xs" c="dimmed">
            •
          </Text>

          <Group gap={4}>
            <IconCalendar size={14} className="text-white" />
            <Text size="xs" c="white">
              {new Date(video.uploadedAt).toLocaleDateString(undefined, {
                year: "numeric",
                month: "long",
                day: "numeric",
              })}
            </Text>
          </Group>
        </Group>
      </Stack>

      <SimpleGrid cols={2} spacing="xs">
        <StatusCard
          icon={IconEye}
          label="Views"
          value={video.views?.toLocaleString() || "0"}
          color="white"
        />
        <StatusCard
          icon={IconThumbUp}
          label="Likes"
          value={video.likes?.toLocaleString() || "0"}
          color="white"
        />
      </SimpleGrid>

      {tags && tags.length > 0 && (
        <Stack gap="xs">
          <Group gap="xs">
            <IconTag size={16} className="text-white" />
            <Text c="white" size="sm" fw={600}>
              Tags
            </Text>
          </Group>
          <Group gap="xs">
            <VideoItemTags tags={tags} maxVisibleTags={4} />
          </Group>
        </Stack>
      )}

      <Divider color="dark.8" />

      <Group justify="space-between">
        <CopyButton value={video.link}>
          {({ copied, copy }) => (
            <Button
              variant="subtle"
              color={copied ? "teal" : "gray"}
              leftSection={
                copied ? <IconCheck size={16} /> : <IconCopy size={16} />
              }
              onClick={copy}
            >
              Copy URL
            </Button>
          )}
        </CopyButton>

        <Button
          variant="light"
          leftSection={<IconExternalLink size={16} />}
          onClick={() => openUrl(video.link)}
        >
          Open on YouTube
        </Button>
      </Group>
    </Stack>
  );
}

function StatusCard({
  icon: Icon,
  label,
  value,
  color,
}: {
  icon: any;
  label: string;
  value: string;
  color?: string;
}) {
  return (
    <Stack
      gap={4}
      align="center"
      className="rounded-md bg-stone-900/75 p-3 transition-colors hover:bg-stone-900"
    >
      <Group gap={6}>
        <Icon size={18} className="text-dimmed" />
        <Text size="md" fw={600} c={color}>
          {value}
        </Text>
      </Group>
      <Text size="xs" c="dimmed">
        {label}
      </Text>
    </Stack>
  );
}
