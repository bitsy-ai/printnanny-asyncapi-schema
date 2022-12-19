import {GstreamerCaps} from './GstreamerCaps';
import {CameraSourceType} from './CameraSourceType';
export interface Camera {
  availableCaps: GstreamerCaps[];
  index: number;
  device_name: string;
  label: string;
  src_type: CameraSourceType;
}
