import {PrintNannyDetectionSettings} from './PrintNannyDetectionSettings';
import {HlsSettings} from './HlsSettings';
import {Camera} from './Camera';
import {PlaybackVideo} from './PlaybackVideo';
export interface PrintNannyCameraSettings {
  overlay_udp_port: number;
  record_video: boolean;
  cloud_backup: boolean;
  preview: boolean;
  video_framerate: number;
  video_udp_port: number;
  detection: PrintNannyDetectionSettings;
  hls: HlsSettings;
  video_src: Camera | PlaybackVideo;
}
