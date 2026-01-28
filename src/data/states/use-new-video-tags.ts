import { parseAsArrayOf, parseAsInteger, useQueryState } from "nuqs";

export const useNewVideoTags = () => {
  return useQueryState(
    "new-video-tags",
    parseAsArrayOf(parseAsInteger).withDefault([]),
  );
};
