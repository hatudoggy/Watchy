// ===========================================
// Commands
// ===========================================

export const ytPreviewKeys = {
  get: (link: string) => ["youtube-preview", link],
};

export const videoKeys = {
  all: () => ["videos"] as const,
  list: (filter?: unknown, sort?: unknown) =>
    ["videos", "list", filter, sort] as const,
  get: (id: number) => ["videos", id],
};

export const channelKeys = {
  all: () => ["channels"] as const,
  get: (id: number) => ["channels", id],
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
