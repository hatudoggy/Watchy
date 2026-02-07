import { useQuery } from "@tanstack/react-query";
import { channelKeys } from "../query-keys";
import { getChannelDetails } from "../../commands/database";

export const useChannel = (id: number) => {
  return useQuery({
    queryKey: channelKeys.get(id),
    queryFn: () => getChannelDetails({ id }),
    enabled: id != -1,
    meta: {
      successMessage: `Successfully fetched channel with id: ${id}`,
      errorMessage: `Error fetching channel with id: ${id}`,
      log: true,
    },
  });
};
