import {SystemdUnitChangeState} from './SystemdUnitChangeState';
interface SystemdUnitChange {
  change: SystemdUnitChangeState;
  file: string;
  destination: string;
}
export { SystemdUnitChange };