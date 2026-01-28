import { ActionIcon as MantineActionIcon } from "@mantine/core";
import { ComponentPropsWithRef } from "react";
import { BasicColors, InputSizes, Sizes, Variants } from "./types";
import { cn } from "@/utils/utils";

export interface IconButtonProps extends ComponentPropsWithRef<"button"> {
  className?: string;
  color?: BasicColors | string;
  radius?: Sizes;
  variant?: Variants;
  size?: Sizes | InputSizes | string;
}

export default function IconButton({
  children,
  className,
  radius,
  color,
  size,
  variant,
  ...props
}: IconButtonProps) {
  return (
    <MantineActionIcon
      className={cn(className)}
      variant={variant}
      size={size}
      color={color}
      radius={radius}
      {...props}
    >
      {children}
    </MantineActionIcon>
  );
}
