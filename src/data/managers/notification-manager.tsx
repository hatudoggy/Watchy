import { notifications } from "@mantine/notifications";
import { IconCheck, IconX } from "@tabler/icons-react";

export const notificationOptions = {
  autoClose: 1000,
  position: "bottom-right",
} as const;

export const showOkNotification = (title: string, message: string) => {
  const id = crypto.randomUUID();

  notifications.show({
    id,
    title: title,
    message: message,
    color: "teal",
    icon: <IconCheck />,
    onClick: () => {
      notifications.hide(id);
    },
  });
};

export const showErrorNotification = (title: string, message: string) => {
  const id = crypto.randomUUID();

  notifications.show({
    id,
    title: title,
    message: message,
    color: "red",
    icon: <IconX />,
    onClick: () => {
      notifications.hide(id);
    },
  });
};
