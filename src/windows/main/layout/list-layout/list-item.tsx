import Text from "@/components/ui/text";
import { Tooltip } from "@mantine/core";
import { Tag, VideoStatus } from "@/data/types/database.types";
import { openUrl } from "@tauri-apps/plugin-opener";
import ChannelTooltip from "./channel-tooltip";
import VideoItemTags from "../../components/video-item-tags";
import { useContextMenu } from "mantine-contextmenu";
import { MouseEvent } from "react";
import { IconEdit, IconInfoCircle, IconTrash } from "@tabler/icons-react";
import { useDeleteVideo } from "@/data/hooks/mutations/video-mutation";
import VideoThumbnail from "@/windows/main/components/video-thumbnail";
import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";
import { modals } from "@mantine/modals";
import { modalsManager } from "../../modals/modals-manager";

interface ListItemProps {
  id: number;
  link: string;
  thumbnail: string;
  title: string;
  channel: {
    id: number;
    link: string;
    name: string;
  };
  duration: number;
  status: VideoStatus;
  tags: Tag[];
}

export default function ListItem({
  id,
  link,
  thumbnail,
  title,
  channel,
  duration,
  tags,
}: ListItemProps) {
  const { mutate: remove } = useDeleteVideo();
  const { showContextMenu } = useContextMenu();

  const contextMenu = (
    e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>,
    // tag: Tag,
  ) =>
    showContextMenu(
      [
        {
          key: "details",
          icon: <IconInfoCircle size={16} />,
          title: "Details",
          onClick: () => modalsManager.videoDetails({ videoId: id }),
        },
        {
          key: "edit",
          icon: <IconEdit size={16} />,
          title: "Edit",
          onClick: () => modalsManager.videoEdit({ videoId: id }),
        },
        {
          key: "delete",
          color: "red",
          icon: <IconTrash size={16} />,
          title: "Delete",
          onClick: () => handleRemove(),
        },
      ],
      {
        className: "rounded-md shadow-xl min-w-32",
        style: { border: "none" },
      },
    )(e);

  const handleRemove = () => {
    modals.openConfirmModal({
      title: "Delete Video?",
      centered: true,
      size: "sm",
      children: (
        <Text size="sm">Are you sure you want to delete this video?</Text>
      ),
      labels: { confirm: "Confirm", cancel: "Cancel" },
      confirmProps: { color: "red" },
      onConfirm: () => remove({ id }),
    });
  };

  const handleOpenUrl = (
    e: MouseEvent<
      HTMLImageElement | HTMLParagraphElement,
      globalThis.MouseEvent
    >,
    link: string,
  ) => {
    e.preventDefault();
    openUrl(link);
  };

  return (
    <Group
      className="h-28 gap-4 rounded-md p-2 py-3 hover:bg-neutral-800"
      onContextMenu={(e) => contextMenu(e)}
    >
      <VideoThumbnail
        className="cursor-pointer"
        src={thumbnail}
        duration={duration}
        onClick={(e) => handleOpenUrl(e, link)}
      />
      <Stack className="flex-1 justify-between">
        <Stack className="gap-0.5">
          <Tooltip label={title} color="gray" openDelay={500}>
            <Text
              span
              style={{ display: "inline-block", maxWidth: "fit-content" }}
            >
              <Text
                className="line-clamp-2 leading-none hover:cursor-pointer"
                style={{ lineHeight: 1 }}
                size="md"
                fw={500}
                c="white"
                onClick={(e) => handleOpenUrl(e, link)}
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
              <Text
                className="line-clamp-1 hover:cursor-pointer"
                size="sm"
                onClick={(e) => handleOpenUrl(e, channel.link)}
              >
                {channel.name}
              </Text>
            </Text>
          </Tooltip>
        </Stack>
        <Stack>
          <VideoItemTags tags={tags} />
        </Stack>
      </Stack>
    </Group>
  );
}
