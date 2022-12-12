import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
export interface SystemdManagerGetUnitFileStateError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerGetUnitRequest;
}
