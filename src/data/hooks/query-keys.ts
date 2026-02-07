// ===========================================
// Commands
// ===========================================

export const ytPreviewKeys = {
  get: (link: string) => ["youtube-preview", link] as const,
};

export const videoKeys = {
  all: () => ["videos"] as const,
  list: (filter?: unknown, sort?: unknown) =>
    ["videos", "list", filter, sort] as const,
  get: (id: number) => ["videos", id] as const,
};

export const channelKeys = {
  all: () => ["channels"] as const,
  get: (id: number) => ["channels", id] as const,
};

export const tagKeys = {
  all: () => ["tags"] as const,
};

// ===========================================
// Settings
// ===========================================

export const settingsKeys = {
  all: () => ["settings"] as const,
  // theme: () => ["settings", "theme"],
};
