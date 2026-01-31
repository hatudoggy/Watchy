import { useVirtualizer } from "@tanstack/react-virtual";
import { useRef } from "react";

export const useListVirtualization = (length: number) => {
  const listRef = useRef(null);

  const listVirtualizer = useVirtualizer({
    count: length,
    getScrollElement: () => listRef.current,
    estimateSize: () => 112,
    overscan: 5,
  });

  return {
    listRef,
    listVirtualizer,
  };
};
