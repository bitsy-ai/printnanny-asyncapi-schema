import {VideoRecordingStatus} from './VideoRecordingStatus';
export interface VideoRecording {
  id: string;
  recording_status: VideoRecordingStatus;
  recording_start?: string;
  recording_end?: string;
  mp4_file_name: string;
  gcode_file_minus_name?: string;
  cloud_sync_status: VideoRecordingStatus;
  cloud_sync_start?: string;
  cloud_sync_end?: string;
}
