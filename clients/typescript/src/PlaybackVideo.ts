import {PlaybackSourceType} from './PlaybackSourceType';
export interface PlaybackVideo {
  cover: string;
  display_name: string;
  uri: string;
  src_type: PlaybackSourceType;
}
