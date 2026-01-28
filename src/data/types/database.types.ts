// =====================
// ENUMS
// =====================

export const VideoStatuses = ["SAVED", "TOWATCH", "WATCHED"] as const;

export type VideoStatus = (typeof VideoStatuses)[number];

export enum VideoSort {
  CreatedAtDesc = "CreatedAtDesc",
  CreatedAtAsc = "CreatedAtAsc",
  UpdatedAtDesc = "UpdatedAtDesc",
  UpdatedAtAsc = "UpdatedAtAsc",
  UploadedAtDesc = "UploadedAtDesc",
  UploadedAtAsc = "UploadedAtAsc",
  WatchedAtDesc = "WatchedAtDesc",
  WatchedAtAsc = "WatchedAtAsc",
  DurationDesc = "DurationDesc",
  DurationAsc = "DurationAsc",
  TitleAsc = "TitleAsc",
  TitleDesc = "TitleDesc",
}

// =====================
// SCHEMA TYPES
// =====================

export interface Video {
  id: number;
  channelId: number;
  link: string;
  title: string;
  thumbnail: string;
  duration: number | null;
  views: number | null;
  likes: number | null;
  uploadedAt: string;
  status: VideoStatus;
  createdAt: string;
  updatedAt: string;
  watchedAt: string | null;
}

export interface Tag {
  id: number;
  name: string;
  color: string;
  createdAt: string;
  updatedAt: string;
}

export interface Channel {
  id: number;
  link: string;
  name: string;
  avatar: string;
  verified: boolean;
  subscribers: number | null;
  createdAt: string;
  updatedAt: string;
}

// =====================
// DTO TYPES
// =====================

export interface VideoFilter {
  channelId?: number | null;
  status?: VideoStatus | null;
  tagIds?: number[];
  search?: string | null;
}

export interface VideoItem {
  id: number;
  channelId: number;
  channelName: string;
  channelLink: string;
  link: string;
  title: string;
  thumbnail: string;
  duration: number | null;
  views: number | null;
  likes: number | null;
  uploadedAt: string;
  status: VideoStatus;
  createdAt: string;
  updatedAt: string;
  watchedAt: string | null;
}

export interface VideoItemWithTags {
  video: VideoItem;
  tags: Tag[];
}

export interface CreateVideo {
  channelId: number;
  link: string;
  title: string;
  thumbnail: string;
  duration: number | null;
  views: number | null;
  likes: number | null;
  uploadedAt: string;
  status: VideoStatus;
}

export interface UpdateVideo {
  link?: string;
  title?: string;
  thumbnail?: string;
  duration?: number | null;
  views?: number | null;
  likes?: number | null;
  uploadedAt?: string;
  status?: VideoStatus;
}

export interface CreateChannel {
  link: string;
  name: string;
  avatar: string;
  verified: boolean;
  subscribers: number | null;
}

export interface UpdateChannel {
  link: string;
  name: string;
  avatar: string;
  verified: boolean;
  subscribers: number | null;
}

export interface CreateTag {
  name: string;
  color: string;
}

export interface UpdateTag {
  name: string;
  color: string;
}
