
export interface VideoRecording {
  id: string;
  cloud_sync_done: boolean;
  dir: string;
  gcode_file_name?: string;
  recording_start?: string;
  recording_end?: string;
}
