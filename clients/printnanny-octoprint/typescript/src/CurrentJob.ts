import {Job} from './Job';
import {JobProgress} from './JobProgress';
export interface CurrentJob {
  job: Job;
  progress: JobProgress;
  state: string;
  error?: string;
}
