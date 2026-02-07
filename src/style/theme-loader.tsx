import { getSettings } from "@/data/managers/settings-manager";
import { useMantineColorScheme } from "@mantine/core";
import { useEffect } from "react";

export default function ThemeLoader() {
  const { setColorScheme } = useMantineColorScheme();

  const loadTheme = async () => {
    const settings = await getSettings();

    if (settings.theme) setColorScheme(settings.theme);
  };

  useEffect(() => {
    loadTheme();
  }, []);

  return <></>;
}
