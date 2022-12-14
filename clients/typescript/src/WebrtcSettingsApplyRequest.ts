import {Camera} from './Camera';
import {PlaybackVideo} from './PlaybackVideo';
export interface WebrtcSettingsApplyRequest {
  video_src?: Camera | PlaybackVideo;
}
