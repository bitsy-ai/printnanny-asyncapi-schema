import {GcodeFile} from './GcodeFile';
interface Job {
  file?: GcodeFile;
  estimatedPrintTime?: string;
  lastPrintTime?: string;
  filamentLength?: string;
  filamentVolume?: string;
}
export { Job };