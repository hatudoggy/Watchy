import {
  Menu as MantineMenu,
  MenuProps as MantineMenuProps,
} from "@mantine/core";

export interface MenuProps extends MantineMenuProps {
  className?: string;
}

export default function Menu({ children, className, ...props }: MenuProps) {
  return <MantineMenu {...props}>{children}</MantineMenu>;
}
