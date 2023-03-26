import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
interface SystemdManagerGetUnitFileStateError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerGetUnitRequest;
}
export { SystemdManagerGetUnitFileStateError };