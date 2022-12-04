import {SystemdUnit} from './SystemdUnit';
export interface SystemdManagerStartUnitReply {
  job: string;
  unit: SystemdUnit;
}
