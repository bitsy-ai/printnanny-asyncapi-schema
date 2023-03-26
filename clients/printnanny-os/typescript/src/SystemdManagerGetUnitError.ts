import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
interface SystemdManagerGetUnitError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerGetUnitRequest;
}
export { SystemdManagerGetUnitError };