import { Image as MantineImage } from "@mantine/core";
import { ComponentPropsWithRef } from "react";
import { Sizes } from "./types";
import { cn } from "@/utils/utils";

export interface ImageProps extends ComponentPropsWithRef<"img"> {
  fallbackSrc?: string;
  radius?: Sizes | string;
  h?: number | string;
  w?: number | string;
  fit?: "cover" | "contain";
}

const defaultFallback = "https://placehold.co/600x400?text=Placeholder";

export default function Image({
  className,
  radius,
  h,
  w = "auto",
  fallbackSrc = defaultFallback,
  fit,
  ...props
}: ImageProps) {
  return (
    <MantineImage
      className={cn(className)}
      radius="md"
      h={h}
      w={w}
      fit={fit}
      fallbackSrc={fallbackSrc}
      {...props}
    />
  );
}
