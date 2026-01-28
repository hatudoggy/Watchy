import { useQuery } from "@tanstack/react-query";
import { channelKeys } from "../query-keys";
import { getChannelDetails } from "../../commands/database";

export const useChannel = (id: number) => {
  return useQuery({
    queryKey: channelKeys.get(id),
    queryFn: () => getChannelDetails({ id }),
  });
};
