import { SettingsStore } from "../types/settings.types";
import { invokeTauri } from "./invoke-tauri";

// =====================
// SETTINGS COMMANDS
// =====================

export const setDownloadPath = async (): Promise<string> => {
  return invokeTauri<string>("set_download_path");
};

export const exportData = async (): Promise<void> => {
  return invokeTauri<void>("export_data");
};

export const importData = async (): Promise<SettingsStore> => {
  return invokeTauri<SettingsStore>("import_data");
};
