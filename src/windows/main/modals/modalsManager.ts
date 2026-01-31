import { modals } from "@mantine/modals";
import { MODAL_IDS } from "./modals";
import { VideoDetailsModalProps } from "./video-details.modal";
import { VideoEditModalProps } from "./video-edit.modal";

export const modalsManager = {
  videoDetails(props: VideoDetailsModalProps) {
    return modals.openContextModal({
      modal: MODAL_IDS.videoDetails,
      // title: "Video Details",
      innerProps: props,
      withCloseButton: false,
      overlayProps: {
        backgroundOpacity: 0.55,
        blur: 3,
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
    });
  },

  videoEdit(props: VideoEditModalProps) {
    return modals.openContextModal({
      modal: MODAL_IDS.videoEdit,
      innerProps: props,
      withCloseButton: false,
      overlayProps: {
        backgroundOpacity: 0.55,
        blur: 3,
      },
      centered: true,
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
    });
  },
};
