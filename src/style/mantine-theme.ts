import { createTheme } from "@mantine/core";

export const theme = createTheme({
  components: {
    Notification: {
      defaultProps: {
        radius: "lg",
        withCloseButton: false,
      },
      classNames: {
        root: "before:content-none cursor-pointer w-fit pr-4",
        title: "text-white",
        description: "text-white",
        icon: "bg-white",
      },
      styles: {
        root: {
          backgroundColor:
            "var(--notification-color, var(--mantine-primary-color-filled))",
        },
        icon: {
          color:
            "var(--notification-color, var(--mantine-primary-color-filled))",
        },
      },
    },
  },
});
