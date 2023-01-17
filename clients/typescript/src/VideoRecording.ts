import {VideoRecordingStatus} from './VideoRecordingStatus';
export interface VideoRecording {
  id: string;
  recording_status: VideoRecordingStatus;
  recording_start?: number;
  recording_end?: number;
  recording_file_name: string;
  gcode_file_minus_name?: string;
  cloud_sync_status: VideoRecordingStatus;
  cloud_sync_start?: number;
  cloud_sync_end?: number;
}
