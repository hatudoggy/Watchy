import { useQuery } from "@tanstack/react-query";
import { videoKeys } from "../query-keys";
import { getVideo } from "../../commands/database";

export const useVideo = (id: number) => {
  return useQuery({
    queryKey: videoKeys.get(id),
    queryFn: () => getVideo({ id }),
    placeholderData: (prev) => prev,
  });
};
