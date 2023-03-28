import {Job} from './Job';
import {JobProgress} from './JobProgress';
interface JobProgressChanged {
  job?: Job;
  storage?: string;
  path?: string;
  progress?: JobProgress;
}
export { JobProgressChanged };