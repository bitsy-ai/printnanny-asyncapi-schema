import {JobStatus} from './JobStatus';
export interface JobStatusChanged {
  reserved_status: JobStatus;
}
