import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
export interface SystemdManagerGetUnitError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerGetUnitRequest;
}
