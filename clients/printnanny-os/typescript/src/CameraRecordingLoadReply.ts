import {VideoRecording} from './VideoRecording';
import {VideoRecordingPart} from './VideoRecordingPart';
interface CameraRecordingLoadReply {
  recording?: VideoRecording;
  parts?: VideoRecordingPart[];
}
export { CameraRecordingLoadReply };