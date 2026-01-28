import { getCurrentWindow } from "@tauri-apps/api/window";
import { IconMinus, IconX } from "@tabler/icons-react";

export default function TopBar() {
  const appWindow = getCurrentWindow();

  return (
    <div className="flex h-8 shrink-0 items-center justify-between select-none">
      <div
        className="flex flex-1 items-center px-4 text-xs font-medium text-zinc-400"
        data-tauri-drag-region
      >
        Watchy
      </div>
      <div className="flex h-full">
        <button
          type="button"
          className="inline-flex h-full w-10 items-center justify-center hover:bg-zinc-800 focus:outline-none"
          onClick={() => appWindow.minimize()}
        >
          <IconMinus size={16} className="text-zinc-400" />
        </button>
        <button
          type="button"
          className="group inline-flex h-full w-10 items-center justify-center hover:bg-red-500 focus:outline-none"
          onClick={() => appWindow.close()}
        >
          <IconX size={16} className="text-zinc-400 group-hover:text-white" />
        </button>
      </div>
    </div>
  );
}
