import {OctoPrintJob} from './OctoPrintJob';
import {OctoPrintJobProgress} from './OctoPrintJobProgress';
export interface OctoPrintCurrentJob {
  job: OctoPrintJob;
  progress: OctoPrintJobProgress;
  state: string;
  error?: string;
}
