import {CameraSourceType} from './CameraSourceType';
import {GstreamerCaps} from './GstreamerCaps';
export interface Camera {
  index: number;
  device_name: string;
  label: string;
  src_type: CameraSourceType;
  selected_caps: GstreamerCaps;
  available_caps: GstreamerCaps[];
}
