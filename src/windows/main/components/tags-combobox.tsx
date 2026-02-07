import { useEditTag } from "@/data/hooks/mutations/tag-mutation";
import { Tag } from "@/data/types/database.types";
import {
  Button,
  ColorPicker,
  Combobox,
  Group,
  Loader,
  parseThemeColor,
  Pill,
  PillsInput,
  PillsInputFieldProps,
  Stack,
  Text,
  TextInput,
  useCombobox,
  useMantineTheme,
} from "@mantine/core";
import { IconCheck, IconTags } from "@tabler/icons-react";
import { useContextMenu } from "mantine-contextmenu";
import { MouseEvent, useState } from "react";

interface TagsComboboxProps {
  allTags: Tag[];
  isLoading?: boolean;
  tags: Tag[];
  onAddTag?: (tag: Tag) => void;
  onCreateTag?: (tagName: string) => void;
  onRemoveTag?: (id: number) => void;
  variant?: PillsInputFieldProps["variant"];
  placeholder?: string;
}

export default function TagsCombobox({
  allTags,
  isLoading,
  tags,
  onAddTag,
  onCreateTag,
  onRemoveTag,
  variant,
  placeholder = "Search tags...",
}: TagsComboboxProps) {
  const combobox = useCombobox();
  const { showContextMenu } = useContextMenu();
  const theme = useMantineTheme();

  const [search, setSearch] = useState("");

  const exactMatch = allTags.some(
    (t) => t.name.toLowerCase() === search.trim().toLowerCase(),
  );

  const filteredTags =
    allTags.filter((t) =>
      t.name.toLowerCase().includes(search.toLowerCase().trim()),
    ) || [];

  const contextMenu = (
    e: MouseEvent<HTMLDivElement, globalThis.MouseEvent>,
    tag: Tag,
  ) =>
    showContextMenu((close) => <TagContextMenu tag={tag} close={close} />, {
      className: "rounded-md shadow-xl",
      style: { border: "none" },
    })(e);

  return (
    <Combobox
      store={combobox}
      // withinPortal={false}
      onOptionSubmit={(val) => {
        setSearch("");
        const selectedTag = allTags.find((t) => t.name === val);

        if (selectedTag) {
          onAddTag?.(selectedTag);
        } else {
          onCreateTag?.(val);
        }
      }}
    >
      <Combobox.DropdownTarget>
        <PillsInput variant={variant}>
          <Pill.Group onClick={(e) => e.stopPropagation()}>
            {tags.map((tag) => {
              const parsedColor = parseThemeColor({
                color: tag.color,
                theme,
              });

              return (
                <Pill
                  key={tag.id}
                  className="cursor-default"
                  style={{
                    backgroundColor: tag.color,
                    color: parsedColor.isLight ? "black" : "white",
                  }}
                  withRemoveButton
                  onRemove={() => {
                    onRemoveTag && onRemoveTag(tag.id);
                  }}
                  onMouseDown={(e) => {
                    e.stopPropagation();
                  }}
                  onContextMenu={(e) => contextMenu(e, tag)}
                >
                  {tag.name}
                </Pill>
              );
            })}
            {isLoading ? (
              <Combobox.Empty>
                <Group justify="center" p="xs">
                  <Loader size="sm" />
                </Group>
              </Combobox.Empty>
            ) : (
              " "
            )}
            {tags.length === 0 && <IconTags opacity={0.8} size={16} />}
            <Combobox.EventsTarget>
              <PillsInput.Field
                className="flex-1"
                onClick={() => combobox.openDropdown()}
                onFocus={() => combobox.openDropdown()}
                onBlur={() => combobox.closeDropdown()}
                placeholder={placeholder}
                value={search}
                onChange={(e) => setSearch(e.target.value)}
              />
            </Combobox.EventsTarget>
          </Pill.Group>
        </PillsInput>
      </Combobox.DropdownTarget>

      <Combobox.Dropdown>
        <Combobox.Options>
          {filteredTags.length > 0 ? (
            filteredTags.map((tag) => (
              <Combobox.Option
                value={tag.name}
                key={tag.id}
                active={tags.some((t) => t.id === tag.id)}
                onContextMenu={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  contextMenu(e, tag);
                }}
              >
                <Group gap="sm">
                  <div
                    style={{
                      width: 16,
                      height: 16,
                      minWidth: 16,
                      borderRadius: "50%",
                      backgroundColor: tag.color,
                      display: "flex",
                      alignItems: "center",
                      justifyContent: "center",
                    }}
                  >
                    {tags.some((t) => t.id === tag.id) && (
                      <IconCheck size={10} color="white" stroke={3} />
                    )}
                  </div>
                  <span>{tag.name}</span>
                </Group>
              </Combobox.Option>
            ))
          ) : (
            <Combobox.Empty>Nothing found</Combobox.Empty>
          )}

          {!exactMatch && search.trim().length > 0 && (
            <Combobox.Option value={search.trim()}>
              + Create "{search.trim()}"
            </Combobox.Option>
          )}
        </Combobox.Options>
      </Combobox.Dropdown>
    </Combobox>
  );
}

interface TagContextMenuProps {
  tag: Tag;
  close: () => void;
}

const swatches = [
  { name: "Red", color: "#fa5252" },
  { name: "Orange", color: "#fd7e14" },
  { name: "Yellow", color: "#fab005" },
  { name: "Green", color: "#40c057" },
  { name: "Blue", color: "#228be6" },
  { name: "Indigo", color: "#4c6ef5" },
  { name: "Grape", color: "#be4bdb" },
];

function TagContextMenu({ tag, close }: TagContextMenuProps) {
  const [color, setColor] = useState(tag.color);
  const [name, setName] = useState(tag.name);

  const { mutate } = useEditTag();

  const handleEditTag = () => {
    mutate({
      id: tag.id,
      tag: {
        name: name,
        color: color,
      },
    });
    close();
  };

  return (
    <Stack
      p="xs"
      w={260}
      gap="sm"
      onClick={(e) => {
        e.preventDefault();
        e.stopPropagation();
      }}
    >
      <TextInput
        label="Name"
        defaultValue={tag.name}
        placeholder="Tag Name"
        value={name}
        onChange={(e) => setName(e.target.value)}
        data-autofocus
      />

      <Stack gap={4}>
        <Text size="sm" fw={500}>
          Color
        </Text>
        <ColorPicker
          format="hex"
          value={color}
          onChange={setColor}
          size="xs"
          swatches={swatches.map((s) => s.color)}
          fullWidth
        />
      </Stack>

      <Group justify="flex-end" gap="xs">
        <Button size="xs" variant="default" onClick={close}>
          Cancel
        </Button>
        <Button size="xs" onClick={() => handleEditTag()}>
          Save
        </Button>
      </Group>
    </Stack>
  );
}
