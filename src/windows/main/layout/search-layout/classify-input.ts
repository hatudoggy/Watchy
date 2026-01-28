type SearchInputKind =
  | { type: "empty" }
  | { type: "invalid" }
  | { type: "youtube-url"; videoId: string }
  | { type: "url"; url: string }
  | { type: "text"; query: string };

export const classifyInput = (raw: string): SearchInputKind => {
  const value = raw.trim();

  if (!value) {
    return { type: "empty" };
  }

  try {
    const url = new URL(value);

    const ytId = url.hostname.includes("youtube.com")
      ? url.searchParams.get("v")
      : url.hostname === "youtu.be"
        ? url.pathname.slice(1)
        : null;

    if (ytId) {
      return { type: "youtube-url", videoId: ytId };
    }

    return { type: "url", url: value };
  } catch {
    return { type: "text", query: value };
  }
};
