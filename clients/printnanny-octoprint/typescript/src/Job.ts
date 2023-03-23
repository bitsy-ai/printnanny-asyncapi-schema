import {GcodeFile} from './GcodeFile';
interface Job {
  file?: GcodeFile;
  estimated_print_time?: string;
  last_print_time?: string;
  filament_length?: string;
  filament_volume?: string;
}
export { Job };