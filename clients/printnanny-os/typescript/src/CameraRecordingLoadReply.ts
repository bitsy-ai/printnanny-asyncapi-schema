import {VideoRecording} from './VideoRecording';
import {VideoRecordingPart} from './VideoRecordingPart';
export interface CameraRecordingLoadReply {
  recording?: VideoRecording;
  parts?: VideoRecordingPart[];
}
