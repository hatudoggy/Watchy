export interface YoutubeMetadata {
  video: YoutubeVideoMetadata;
  channel: YoutubeChannelMetadata;
}

interface YoutubeVideoMetadata {
  link: string;
  title: string;
  thumbnail: string;
  uploadDate: string;

  duration?: number;
  views?: number;
  likes?: number;
}

interface YoutubeChannelMetadata {
  name: string;
  avatar: string;
  subscribers: number;
  verified: boolean;
}
