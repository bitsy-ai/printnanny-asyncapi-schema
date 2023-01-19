import {VideoRecordingStatus} from './VideoRecordingStatus';
export interface VideoRecording {
  id: string;
  recording_status: VideoRecordingStatus;
  recording_start?: string;
  recording_end?: string;
  mp4_file_name: string;
  mp4_download_url?: string;
  mp4_upload_url?: string;
  gcode_file_name?: string;
  cloud_sync_status: VideoRecordingStatus;
  cloud_sync_start?: string;
  cloud_sync_end?: string;
}
