import {Job} from './Job';
import {JobProgress} from './JobProgress';
interface CurrentJob {
  job: Job;
  progress: JobProgress;
  state: string;
  error?: string;
}
export { CurrentJob };