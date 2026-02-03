import SettingsModal from "./settings.modal";
import VideoDetailsModal from "./video-details.modal";
import VideoEditModal from "./video-edit.modal";

declare module "@mantine/modals" {
  export interface MantineModalsOverride {
    modals: typeof modals;
  }
}

export const MODAL_IDS = {
  videoDetails: "videoDetails",
  videoEdit: "videoEdit",
  settings: "settings",
} as const;

export type ModalId = keyof typeof MODAL_IDS;

export const modals = {
  videoDetails: VideoDetailsModal,
  videoEdit: VideoEditModal,
  settings: SettingsModal,
};
