import { YoutubeMetadata } from "../types/youtube-parser.types";
import { invokeTauri } from "./invoke-tauri";

export const fetchYoutubeMetadata = async (params: {
  url: string;
}): Promise<YoutubeMetadata> => {
  const data: YoutubeMetadata = await invokeTauri("fetch_youtube_metadata", {
    url: params.url,
  });

  return data;
};
