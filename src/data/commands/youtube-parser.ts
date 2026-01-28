import { YoutubeMetadata } from "../types/youtube-parser.types";
import { invokeTauri } from "./invokeTauri";

export const fetchYoutubeMetadata = async (
  url: string,
): Promise<YoutubeMetadata> => {
  const data: YoutubeMetadata = await invokeTauri("fetch_youtube_metadata", {
    url,
  });

  return data;
};
