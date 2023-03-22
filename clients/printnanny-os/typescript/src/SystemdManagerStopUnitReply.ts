import {SystemdUnit} from './SystemdUnit';
export interface SystemdManagerStopUnitReply {
  job: string;
  unit: SystemdUnit;
}
