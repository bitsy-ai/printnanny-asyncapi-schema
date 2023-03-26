import {SystemdUnitFileState} from './SystemdUnitFileState';
import {SystemdUnit} from './SystemdUnit';
interface SystemdUnitFileStateChanged {
  unit_file_state: SystemdUnitFileState;
  unit: SystemdUnit;
}
export { SystemdUnitFileStateChanged };