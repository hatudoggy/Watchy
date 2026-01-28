import { cn } from "@/utils/utils";
import { Text as MantineText } from "@mantine/core";
import { ComponentPropsWithRef } from "react";
import { Sizes } from "./types";

export interface Text extends ComponentPropsWithRef<"p"> {
  size?: Sizes;
  fw?: number;
  c?: "black" | "white" | string;
  span?: boolean;
}

export default function Text({
  children,
  className,
  size,
  fw,
  c,
  span,
  ...props
}: Text) {
  return (
    <MantineText
      span
      className={cn(className)}
      size={size}
      fw={fw}
      c={c}
      {...props}
    >
      {children}
    </MantineText>
  );
}
