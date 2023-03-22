import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnitLoadState} from './SystemdUnitLoadState';
import {SystemdUnitFileState} from './SystemdUnitFileState';
export interface SystemdUnit {
  id: string;
  fragment_path: string;
  active_state: SystemdUnitActiveState;
  load_state: SystemdUnitLoadState;
  unit_file_state: SystemdUnitFileState;
}
