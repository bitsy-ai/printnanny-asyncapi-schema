import {SystemdUnitFileState} from './SystemdUnitFileState';
import {SystemdUnit} from './SystemdUnit';
export interface SystemdUnitFileStateChanged {
  unit_file_state: SystemdUnitFileState;
  unit: SystemdUnit;
}
