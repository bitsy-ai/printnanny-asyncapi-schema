import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
export interface SystemdManagerEnableUnitsError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerUnitFilesRequest;
}
