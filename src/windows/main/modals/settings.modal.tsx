import Group from "@/components/ui/group";
import Stack from "@/components/ui/stack";
import Text from "@/components/ui/text";
import {
  Button,
  Center,
  MantineColorScheme,
  SegmentedControl,
  TextInput,
  Title,
  useMantineColorScheme,
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
import { useSettings } from "@/data/hooks/queries/use-settings";
import { useEditSettings } from "@/data/hooks/mutations/use-edit-settings";
import { open } from "@tauri-apps/plugin-dialog";

export interface SettingsModalProps {}

export default function SettingsModal({
  id,
  context,
}: ContextModalProps<SettingsModalProps>) {
  return (
    <Stack className="overflow-x-hidden" gap="lg">
      <Group justify="space-between">
        <Title className="text-white" order={2}>
          Settings
        </Title>
        <IconButton
          radius="xl"
          variant="subtle"
          color="white"
          onClick={() => context.closeModal(id)}
        >
          <IconX />
        </IconButton>
      </Group>

      <Stack className="gap-6">
        <AppearanceSection />
        <DataManagementSection />
        <DownloadPathSection />
      </Stack>
    </Stack>
  );
}

interface ItemLabelProps {
  label: string;
}

function ItemLabel({ label }: ItemLabelProps) {
  return (
    <Text className="text-white" size="sm" fw={500}>
      {label}
    </Text>
  );
}

function AppearanceSection() {
  const { data } = useSettings();
  const { mutate } = useEditSettings();
  const { setColorScheme } = useMantineColorScheme({
    keepTransitions: true,
  });

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

  const handleChangeAppearance = async (value: MantineColorScheme) => {
    mutate({ key: "theme", value: value });
    setColorScheme(value);
  };

  if (!data) return;

  return (
    <Stack gap="xs">
      <ItemLabel label="Appearance" />
      <SegmentedControl
        fullWidth
        radius="md"
        size="md"
        withItemsBorders={false}
        color="blue"
        value={data.theme}
        onChange={(val) => handleChangeAppearance(val as MantineColorScheme)}
        data={appearanceStatus}
        classNames={{
          root: "bg-white/10 border border-white/10",
          indicator: "bg-blue-600 shadow-lg",
          label:
            "text-stone-300 hover:text-white data-[active]:text-white transition-colors",
        }}
      />
    </Stack>
  );
}

function DataManagementSection() {
  return (
    <Stack gap="xs">
      <ItemLabel label="Data Management" />

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
  );
}

function DownloadPathSection() {
  const { data } = useSettings();
  const { mutate } = useEditSettings();

  const handleSelectPath = async () => {
    const path = await open({
      multiple: false,
      directory: true,
    });

    if (!path) return;

    mutate({ key: "download-path", value: path });
  };

  if (!data) return;

  return (
    <Stack gap="xs">
      <ItemLabel label="Downloads" />

      <TextInput
        // label="Video Download Path"
        variant="unstyled"
        placeholder="Select a folder..."
        classNames={{
          input:
            "text-white rounded-md border bg-white/10 border-white/10 bg-white/5 px-2 focus:border-white/40 cursor-pointer",
        }}
        readOnly
        value={data.downloadPath}
        onClick={() => handleSelectPath()}
      />
    </Stack>
  );
}
