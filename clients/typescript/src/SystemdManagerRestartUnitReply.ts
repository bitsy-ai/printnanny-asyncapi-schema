import {SystemdUnit} from './SystemdUnit';
export interface SystemdManagerRestartUnitReply {
  job: string;
  unit: SystemdUnit;
}
