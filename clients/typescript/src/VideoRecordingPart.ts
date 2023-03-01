
export interface VideoRecordingPart {
  id: string;
  part: number;
  size: number;
  deleted: boolean;
  sync_start?: string;
  sync_end?: string;
  file_name: string;
  video_recording_id: string;
}
