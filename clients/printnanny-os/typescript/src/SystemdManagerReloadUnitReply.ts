import {SystemdUnit} from './SystemdUnit';
export interface SystemdManagerReloadUnitReply {
  job: string;
  unit: SystemdUnit;
}
