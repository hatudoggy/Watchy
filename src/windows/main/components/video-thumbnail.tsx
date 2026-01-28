import { formatDuration } from "@/utils/formatter";
import Badge from "@/components/ui/badge";
import Image from "@/components/ui/image";
import { MouseEvent } from "react";
import { cn } from "@/utils/utils";

interface VideoThumbnailProps {
  className?: string;
  src: string;
  duration?: number;
  onClick?: (
    event: MouseEvent<HTMLImageElement, globalThis.MouseEvent>,
  ) => void;
}

export default function VideoThumbnail({
  className,
  src,
  duration,
  onClick,
}: VideoThumbnailProps) {
  return (
    <div className="relative">
      <Image
        className={cn("aspect-video", className)}
        style={{ imageRendering: "auto" }}
        radius="md"
        h={84}
        // w={150}
        fit="cover"
        src={src}
        onClick={onClick}
      />
      {duration && (
        <Badge
          className="absolute right-1 bottom-1 bg-neutral-900/60"
          variant="light"
          size="xs"
          color="white"
        >
          {formatDuration(duration)}
        </Badge>
      )}
    </div>
  );
}
