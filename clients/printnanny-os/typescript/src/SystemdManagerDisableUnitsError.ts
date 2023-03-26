import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
interface SystemdManagerDisableUnitsError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerUnitFilesRequest;
}
export { SystemdManagerDisableUnitsError };