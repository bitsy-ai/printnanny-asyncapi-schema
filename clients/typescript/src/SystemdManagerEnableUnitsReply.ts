import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export interface SystemdManagerEnableUnitsReply {
  request: SystemdManagerUnitFilesRequest;
  changes: SystemdUnitChange[];
}
