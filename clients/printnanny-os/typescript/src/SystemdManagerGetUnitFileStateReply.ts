import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
import {SystemdUnitFileState} from './SystemdUnitFileState';
export interface SystemdManagerGetUnitFileStateReply {
  request: SystemdManagerGetUnitRequest;
  unit_file_state: SystemdUnitFileState;
}
