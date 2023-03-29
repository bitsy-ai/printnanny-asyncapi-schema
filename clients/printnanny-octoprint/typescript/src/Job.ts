import {GcodeFile} from './GcodeFile';
import {Filament} from './Filament';
interface Job {
  file: GcodeFile;
  averagePrintTime?: number;
  estimatedPrintTime: number;
  lastPrintTime: number;
  filaments?: Filament[];
}
export { Job };