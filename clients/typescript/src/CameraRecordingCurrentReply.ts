import {VideoRecording} from './VideoRecording';
export interface CameraRecordingCurrentReply {
  current_recording?: VideoRecording;
  additionalProperties?: Map<string, any>;
}
