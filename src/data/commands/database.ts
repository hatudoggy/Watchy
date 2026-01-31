import { invokeTauri } from "./invokeTauri";
import {
  Channel,
  CreateTag,
  Tag,
  UpdateTag,
  UpdateVideo,
  Video,
  VideoFilter,
  VideoItemWithTags,
  VideoSort,
} from "../types/database.types";
import { YoutubeMetadata } from "../types/youtube-parser.types";

// =====================
// VIDEO COMMANDS
// =====================

export const listVideos = async (params: {
  filter?: VideoFilter;
  sort?: VideoSort;
}): Promise<VideoItemWithTags[]> => {
  return invokeTauri<VideoItemWithTags[]>("list_videos", {
    filter: params.filter ?? null,
    sort: params.sort ?? null,
  });
};

export const addVideo = async (params: {
  metadata: YoutubeMetadata;
  tags?: Tag[];
}): Promise<Video> => {
  return invokeTauri<Video>("add_video", {
    metadata: params.metadata,
    tags: params.tags ?? null,
  });
};

export const getVideo = async (params: {
  id: number;
}): Promise<VideoItemWithTags> => {
  return invokeTauri<VideoItemWithTags>("get_video", params);
};

export const editVideo = async (params: {
  id: number;
  video: UpdateVideo;
}): Promise<Video> => {
  return invokeTauri<Video>("edit_video", params);
};

export const deleteVideo = async (params: { id: number }): Promise<void> => {
  return invokeTauri<void>("delete_video", params);
};

export const addTagToVideo = async (params: {
  videoId: number;
  name: string;
}): Promise<Tag> => {
  return invokeTauri<Tag>("add_tag_to_video", params);
};

export const removeTagFromVideo = async (params: {
  videoId: number;
  tagId: number;
}): Promise<void> => {
  return invokeTauri<void>("remove_tag_from_video", params);
};

export const getChannelDetails = async (params: {
  id: number;
}): Promise<Channel> => {
  return invokeTauri<Channel>("get_channel_details", params);
};

// =====================
// TAG COMMANDS
// =====================

export const listTags = async (): Promise<Tag[]> => {
  return invokeTauri<Tag[]>("list_tags");
};

export const addTag = async (params: { tag: CreateTag }): Promise<Tag> => {
  return invokeTauri<Tag>("add_tag", params);
};

export const editTag = async (params: {
  id: number;
  tag: UpdateTag;
}): Promise<Tag> => {
  return invokeTauri<Tag>("edit_tag", params);
};

export const deleteTag = async (params: { id: number }): Promise<void> => {
  return invokeTauri<void>("delete_tag", params);
};
