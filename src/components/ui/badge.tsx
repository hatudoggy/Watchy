import { Badge as MantineBadge } from "@mantine/core";
import { ComponentPropsWithRef } from "react";
import { BasicColors, Sizes, Variants } from "./types";
import { cn } from "@/utils/utils";

export interface BadgeProps extends ComponentPropsWithRef<"div"> {
  className?: string;
  color?: BasicColors | string;
  radius?: Sizes;
  size?: Sizes | string;
  variant?: Exclude<Variants, "subtle"> | "dot";
}

export default function Badge({
  children,
  className,
  radius,
  size,
  variant,
  color,
}: BadgeProps) {
  return (
    <MantineBadge
      className={cn(className)}
      color={color}
      radius={radius}
      size={size}
      variant={variant}
    >
      {children}
    </MantineBadge>
  );
}
