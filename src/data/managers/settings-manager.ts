import { MantineColorScheme } from "@mantine/core";
import { load } from "@tauri-apps/plugin-store";

export const SETTINGS_FILE = "settings.json";

export type SetSettingsOption =
  | {
      key: "theme";
      value: MantineColorScheme;
    }
  | {
      key: "download-path";
      value: string;
    };

export const getSettings = async () => {
  const store = await load(SETTINGS_FILE);

  const theme = await store.get<MantineColorScheme>("theme");

  if (!theme) throw new Error("Cannot retrieve theme");

  return {
    theme,
  };
};

export const setSettings = async (option: SetSettingsOption) => {
  try {
    const store = await load(SETTINGS_FILE);
    await store.set(option.key, option.value);
  } catch (err) {
    console.error(err);
  }
};
