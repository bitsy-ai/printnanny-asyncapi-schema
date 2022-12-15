import {PlaybackSourceType} from './PlaybackSourceType';
export interface PlaybackVideo {
  cover: string;
  reserved_name: string;
  uri: string;
  src_type: PlaybackSourceType;
}
