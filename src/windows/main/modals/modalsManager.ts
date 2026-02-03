import { modals } from "@mantine/modals";
import { MODAL_IDS } from "./modals";
import { VideoDetailsModalProps } from "./video-details.modal";
import { VideoEditModalProps } from "./video-edit.modal";
import { SettingsModalProps } from "./settings.modal";

const overlayStyle = {
  withCloseButton: false,
  centered: true,
  overlayProps: {
    backgroundOpacity: 0.65,
    blur: 6,
  },
  styles: {
    content: {
      background: "transparent",
      boxShadow: "none",
      border: "none",
      padding: 0,
    },

    body: {
      padding: 0,
    },
  },
};

export const modalsManager = {
  videoDetails(props: VideoDetailsModalProps) {
    return modals.openContextModal({
      modal: MODAL_IDS.videoDetails,
      // title: "Video Details",
      innerProps: props,
      ...overlayStyle,
    });
  },

  videoEdit(props: VideoEditModalProps) {
    return modals.openContextModal({
      modal: MODAL_IDS.videoEdit,
      innerProps: props,
      ...overlayStyle,
    });
  },

  settings(props: SettingsModalProps) {
    return modals.openContextModal({
      modal: MODAL_IDS.settings,
      innerProps: props,
      ...overlayStyle,
    });
  },
};
