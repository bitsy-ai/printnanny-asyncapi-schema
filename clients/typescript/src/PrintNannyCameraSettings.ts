import {PrintNannyDetectionSettings} from './PrintNannyDetectionSettings';
import {Camera} from './Camera';
export interface PrintNannyCameraSettings {
  overlay_udp_port: number;
  record_video: boolean;
  cloud_backup: boolean;
  preview: boolean;
  video_framerate: number;
  video_udp_port: number;
  detection: PrintNannyDetectionSettings;
  snapshot_enabled: boolean;
  snapshot_location: string;
  hls_segments: string;
  hls_playlist: string;
  hls_playlist_root: string;
  hls_enabled: boolean;
  camera: Camera;
}
