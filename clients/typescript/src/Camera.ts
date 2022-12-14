import {VideoSourceType} from './VideoSourceType';
export interface Camera {
  index: number;
  device_name: string;
  label: string;
  video_src: VideoSourceType;
}
