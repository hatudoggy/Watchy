import { useSortVideos } from "@/data/states/use-sort-videos";
import { sortFields, VideoSort } from "@/data/types/database.types";

export const useSortVideosState = () => {
  const [sort, setSort] = useSortVideos();

  const { field, direction } = splitSort(sort);

  const fieldOptions = sortFields.map((v) => ({
    value: v,
    label: v.replace(/([A-Z])/g, " $1").trim(),
  }));

  const directionOptions = [
    { value: "Desc", label: "Desc" },
    { value: "Asc", label: "Asc" },
  ];

  const handleFieldChange = (val: string | null) => {
    if (!val) return;

    setSort(buildSort(val, direction));
  };

  const handleDirectionChange = (val: string | null) => {
    if (!val) return;

    setSort(buildSort(field, val as "Asc" | "Desc"));
  };

  return {
    field,
    direction,
    fieldOptions,
    directionOptions,
    handleFieldChange,
    handleDirectionChange,
  };
};

function splitSort(sort: VideoSort) {
  const match = sort.match(/^(.*)(Asc|Desc)$/);

  if (!match) {
    throw new Error(`Invalid sort: ${sort}`);
  }

  return {
    field: match[1],
    direction: match[2] as "Asc" | "Desc",
  };
}

function buildSort(field: string, direction: "Asc" | "Desc"): VideoSort {
  return `${field}${direction}` as VideoSort;
}
