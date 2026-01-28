export const ytPreviewKeys = {
  get: (link: string) => ["youtube-preview", link],
};

export const videoKeys = {
  all: () => ["videos"] as const,
  list: (filter?: unknown, sort?: unknown) =>
    ["videos", "list", filter, sort] as const,
};

export const channelKeys = {
  get: (id: number) => ["channels", id],
};

export const tagKeys = {
  all: () => ["tags"] as const,
};
