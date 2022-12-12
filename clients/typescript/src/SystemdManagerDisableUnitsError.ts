import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
export interface SystemdManagerDisableUnitsError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerUnitFilesRequest;
}
