import { useQuery } from "@tanstack/react-query";
import { settingsKeys } from "../query-keys";
import { getSettings } from "../../managers/settings-manager";

export const useSettings = () => {
  return useQuery({
    queryKey: settingsKeys.all(),
    queryFn: () => getSettings(),
  });
};
