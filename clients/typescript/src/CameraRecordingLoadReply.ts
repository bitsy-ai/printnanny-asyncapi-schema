import {VideoRecording} from './VideoRecording';
export interface CameraRecordingLoadReply {
  recordings: VideoRecording[];
  current?: VideoRecording;
  additionalProperties?: Map<string, any>;
}
