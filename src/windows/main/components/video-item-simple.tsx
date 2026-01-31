import Text from "@/components/ui/text";
import { Tooltip } from "@mantine/core";
import { VideoStatus } from "@/data/types/database.types";
import ChannelTooltip from "../layout/list-layout/channel-tooltip";
import VideoThumbnail from "@/windows/main/components/video-thumbnail";
import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";

interface VideoItemSimpleProps {
  thumbnail: string;
  title: string;
  channel: {
    id: number;
    link: string;
    name: string;
  };
  duration: number;
  status: VideoStatus;
}

export default function VideoItemSimple({
  thumbnail,
  title,
  channel,
  duration,
}: VideoItemSimpleProps) {
  return (
    <Group className="h-28 gap-4 rounded-md">
      <VideoThumbnail src={thumbnail} duration={duration} />
      <Stack className="flex-1 justify-between">
        <Stack className="gap-0.5">
          <Tooltip label={title} color="gray" openDelay={500}>
            <Text
              span
              style={{ display: "inline-block", maxWidth: "fit-content" }}
            >
              <Text
                className="line-clamp-2 leading-none"
                style={{ lineHeight: 1 }}
                size="md"
                fw={500}
                c="white"
              >
                {title}
              </Text>
            </Text>
          </Tooltip>
          <Tooltip
            label={<ChannelTooltip id={channel.id} />}
            position="bottom-start"
            color="gray"
            openDelay={500}
          >
            <Text
              span
              style={{ display: "inline-block", maxWidth: "fit-content" }}
            >
              <Text className="line-clamp-1" size="sm">
                {channel.name}
              </Text>
            </Text>
          </Tooltip>
        </Stack>
      </Stack>
    </Group>
  );
}
