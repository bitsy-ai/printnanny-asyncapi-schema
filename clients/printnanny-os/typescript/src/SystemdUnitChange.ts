import {SystemdUnitChangeState} from './SystemdUnitChangeState';
export interface SystemdUnitChange {
  change: SystemdUnitChangeState;
  file: string;
  destination: string;
}
