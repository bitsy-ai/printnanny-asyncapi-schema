import {Job} from './Job';
import {JobStatus} from './JobStatus';
interface JobStatusChanged {
  job?: Job;
  reserved_status: JobStatus;
}
export { JobStatusChanged };