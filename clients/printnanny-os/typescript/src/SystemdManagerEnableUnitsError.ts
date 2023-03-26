import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
interface SystemdManagerEnableUnitsError {
  error: string;
  subject_pattern: string;
  request: SystemdManagerUnitFilesRequest;
}
export { SystemdManagerEnableUnitsError };