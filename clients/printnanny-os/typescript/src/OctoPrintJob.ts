import {OctoPrintFile} from './OctoPrintFile';
export interface OctoPrintJob {
  file?: OctoPrintFile;
  estimatedPrintTime?: number;
  lastPrintType?: number;
}
