import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
interface SystemdManagerDisableUnitsReply {
  request: SystemdManagerUnitFilesRequest;
  changes: SystemdUnitChange[];
}
export { SystemdManagerDisableUnitsReply };