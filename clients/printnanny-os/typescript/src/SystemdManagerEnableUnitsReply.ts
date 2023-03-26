import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
interface SystemdManagerEnableUnitsReply {
  request: SystemdManagerUnitFilesRequest;
  changes: SystemdUnitChange[];
}
export { SystemdManagerEnableUnitsReply };