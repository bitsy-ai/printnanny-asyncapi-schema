import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnit} from './SystemdUnit';
interface SystemdUnitActiveStateChanged {
  active_state: SystemdUnitActiveState;
  unit: SystemdUnit;
}
export { SystemdUnitActiveStateChanged };