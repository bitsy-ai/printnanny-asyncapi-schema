import {SystemdManagerUnitFilesRequest} from './SystemdManagerUnitFilesRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export interface SystemdManagerDisableUnitsReply {
  request: SystemdManagerUnitFilesRequest;
  changes: SystemdUnitChange[];
}
