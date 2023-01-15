import {Camera} from './Camera';
import {DetectionSettings} from './DetectionSettings';
import {HlsSettings} from './HlsSettings';
import {RecordingSettings} from './RecordingSettings';
import {RtpSettings} from './RtpSettings';
import {SnapshotSettings} from './SnapshotSettings';
export interface VideoStreamSettings {
  camera: Camera;
  detection: DetectionSettings;
  hls: HlsSettings;
  recording: RecordingSettings;
  rtp: RtpSettings;
  snapshot: SnapshotSettings;
}
