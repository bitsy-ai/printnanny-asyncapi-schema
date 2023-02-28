import {VideoRecording} from './VideoRecording';
import {VideoRecordingPart} from './VideoRecordingPart';
export interface CameraRecordingLoadPartsReply {
  recording?: VideoRecording;
  parts?: VideoRecordingPart[];
}
