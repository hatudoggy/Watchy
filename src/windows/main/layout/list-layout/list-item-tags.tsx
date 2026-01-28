import { Tag } from "@/data/types/database.types";
import { Tooltip } from "@mantine/core";
import Badge from "@/components/ui/badge";
import Group from "@/components/ui/group";
import Text from "@/components/ui/text";
import Stack from "@/components/ui/stack";

interface ListItemTagsProps {
  tags: Tag[];
}

export default function ListItemTags({ tags }: ListItemTagsProps) {
  const maxVisibleTags = 2;
  const visibleTags = tags.slice(0, maxVisibleTags);
  const remainingTags = tags.slice(maxVisibleTags);
  const hasMoreTags = remainingTags.length > 0;

  return (
    <Group className="h-4.5 items-center gap-1">
      {visibleTags.map((tag, index) => (
        <Badge key={index} color={tag.color} size="sm">
          {tag.name}
        </Badge>
      ))}
      {hasMoreTags && (
        <Tooltip
          label={
            <Stack className="gap-1">
              {remainingTags.map((tag, index) => (
                <Text
                  span
                  key={index}
                  size="sm"
                  className="flex items-center gap-2 capitalize"
                >
                  <span
                    style={{
                      width: "8px",
                      height: "8px",
                      borderRadius: "50%",
                      backgroundColor: tag.color,
                      display: "inline-block",
                    }}
                  />
                  {tag.name}
                </Text>
              ))}
            </Stack>
          }
          color="gray"
          withArrow
          miw={100}
        >
          <Text span style={{ display: "inline-flex" }}>
            <Badge
              color="gray"
              variant="light"
              size="sm"
              style={{ cursor: "pointer" }}
            >
              +{remainingTags.length}
            </Badge>
          </Text>
        </Tooltip>
      )}
    </Group>
  );
}
