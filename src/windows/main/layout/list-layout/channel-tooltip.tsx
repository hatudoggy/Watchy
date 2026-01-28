import Image from "@/components/ui/image";
import Text from "@/components/ui/text";
import { IconCircleCheckFilled } from "@tabler/icons-react";
import { formatSubscribers } from "@/utils/formatter";
import { useChannel } from "@/data/hooks/queries/use-channel";
import { Skeleton } from "@mantine/core";
import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";

interface ChannelTooltipProps {
  id: number;
}

export default function ChannelTooltip({ id }: ChannelTooltipProps) {
  const { data, isLoading } = useChannel(id);

  if (isLoading) return <LoadingState />;

  if (!data) return null;

  return (
    <Group className="items-center gap-3 p-1">
      <Image radius="100%" h={40} w={40} fit="cover" src={data.avatar} />
      <Stack className="gap-1">
        <Group className="items-center gap-1">
          <Text size="sm" fw={500} c="white">
            {data.name}
          </Text>
          {data.verified && <IconCircleCheckFilled size={14} />}
        </Group>
        <Text size="xs" c="dimmed">
          {formatSubscribers(data.subscribers || 0)} subscribers
        </Text>
      </Stack>
    </Group>
  );
}

function LoadingState() {
  return (
    <Group className="min-w-45 items-center gap-3 p-1">
      <Skeleton height={40} width={40} circle />
      <Stack className="gap-2">
        <Skeleton height={14} width={100} radius="sm" />
        <Skeleton height={12} width={60} radius="sm" />
      </Stack>
    </Group>
  );
}
