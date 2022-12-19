import {CameraSourceType} from './CameraSourceType';
import {GstreamerCaps} from './GstreamerCaps';
export interface Camera {
  index: number;
  device_name: string;
  label: string;
  src_type: CameraSourceType;
  selectedCaps: GstreamerCaps;
  availableCaps: GstreamerCaps[];
}
