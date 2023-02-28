
export interface VideoRecordingPart {
  id: string;
  part: number;
  size: number;
  deleted: boolean;
  cloud_sync_done: boolean;
  file_name: string;
  video_recording_id: string;
}
