
interface VideoRecordingPart {
  id: string;
  buffer_index: number;
  buffer_runningtime: number;
  size: number;
  deleted: boolean;
  sync_start?: string;
  sync_end?: string;
  file_name: string;
  video_recording_id: string;
}
export { VideoRecordingPart };