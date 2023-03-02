
export interface VideoRecordingPart {
  id: string;
  buffer_index?: number;
  buffer_ts: number;
  buffer_streamtime: number;
  buffer_runningtime: number;
  buffer_duration: number;
  buffer_offset: number;
  buffer_offset_end: number;
  size: number;
  deleted: boolean;
  sync_start?: string;
  sync_end?: string;
  file_name: string;
  video_recording_id: string;
}
