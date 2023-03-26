import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
import {SystemdUnitFileState} from './SystemdUnitFileState';
interface SystemdManagerGetUnitFileStateReply {
  request: SystemdManagerGetUnitRequest;
  unit_file_state: SystemdUnitFileState;
}
export { SystemdManagerGetUnitFileStateReply };