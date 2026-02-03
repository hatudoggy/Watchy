import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";
import Text from "@/components/ui/text";
import {
  Button,
  Center,
  SegmentedControl,
  TextInput,
  Title,
} from "@mantine/core";
import { ContextModalProps } from "@mantine/modals";
import {
  IconDatabaseExport,
  IconDatabaseImport,
  IconMoon,
  IconSun,
  IconX,
} from "@tabler/icons-react";
import IconButton from "@/components/ui/icon-button";

export interface SettingsModalProps {}

export default function SettingsModal({
  id,
  context,
}: ContextModalProps<SettingsModalProps>) {
  const appearanceStatus = [
    {
      value: "light",
      label: (
        <Center style={{ gap: 8 }}>
          <IconSun size={16} />
          <Text span size="sm" fw={500}>
            Light
          </Text>
        </Center>
      ),
    },
    {
      value: "dark",
      label: (
        <Center style={{ gap: 8 }}>
          <IconMoon size={16} />
          <Text span size="sm" fw={500}>
            Dark
          </Text>
        </Center>
      ),
    },
  ];

  return (
    <Stack className="overflow-x-hidden" gap="md">
      <Group justify="space-between">
        <Title order={2}>Settings</Title>
        <IconButton
          radius="xl"
          variant="subtle"
          color="white"
          onClick={() => context.closeModal(id)}
        >
          <IconX />
        </IconButton>
      </Group>

      {/* Theme Section */}
      <Stack gap="xs">
        <Text size="sm" fw={500}>
          Appearance
        </Text>
        <SegmentedControl
          fullWidth
          radius="md"
          size="md"
          withItemsBorders={false}
          color="blue"
          // value={}
          // onChange={}
          data={appearanceStatus}
          classNames={{
            root: "bg-white/10 border border-white/10",
            indicator: "bg-blue-600 shadow-lg",
            label: "text-stone-300 hover:text-white transition-colors",
          }}
        />
      </Stack>

      {/* Data Management Section */}
      <Stack gap="xs">
        <Text size="sm" fw={500}>
          Data Management
        </Text>
        <Group grow>
          <Button
            variant="filled"
            className="bg-white/20"
            leftSection={<IconDatabaseImport size={16} />}
            onClick={() => {}} // Handle Import
          >
            Import Data
          </Button>
          <Button
            variant="filled"
            className="bg-white/20"
            leftSection={<IconDatabaseExport size={16} />}
            onClick={() => {}} // Handle Export
          >
            Export Data
          </Button>
        </Group>
      </Stack>

      {/* Download Path Section */}
      <Stack gap="xs">
        <Text size="sm" fw={500}>
          Downloads
        </Text>

        <TextInput
          // label="Video Download Path"
          variant="unstyled"
          placeholder="Select a folder..."
          classNames={{
            input:
              "rounded-md border bg-white/10 border-white/10 bg-white/5 px-2 focus:border-white/40 cursor-pointer",
          }}
          readOnly
        />
      </Stack>
    </Stack>
  );
}
