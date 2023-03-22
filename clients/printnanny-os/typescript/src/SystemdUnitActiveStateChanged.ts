import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnit} from './SystemdUnit';
export interface SystemdUnitActiveStateChanged {
  active_state: SystemdUnitActiveState;
  unit: SystemdUnit;
}
