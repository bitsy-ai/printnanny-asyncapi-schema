import {Camera} from './Camera';
import {PlaybackVideo} from './PlaybackVideo';
export interface CameraSettingsApplyRequest {
  video_src: Camera | PlaybackVideo;
}
