
export interface VideoRecording {
  id: string;
  recording_file_name: string;
  gcode_file_minus_name?: string;
  ts: number;
  cloud_sync_done: boolean;
}
