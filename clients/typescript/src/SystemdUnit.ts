import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnitLoadState} from './SystemdUnitLoadState';
import {SystemdUnitFileState} from './SystemdUnitFileState';
export interface SystemdUnit {
  id: string;
  fragmentPath: string;
  activeState: SystemdUnitActiveState;
  loadState: SystemdUnitLoadState;
  unitFileState: SystemdUnitFileState;
}
