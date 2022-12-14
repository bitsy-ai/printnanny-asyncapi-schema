import {CameraSourceType} from './CameraSourceType';
export interface Camera {
  index: number;
  device_name: string;
  label: string;
  src_type: CameraSourceType;
}
