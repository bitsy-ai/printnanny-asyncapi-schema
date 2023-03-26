import {SystemdUnit} from './SystemdUnit';
interface SystemdManagerStopUnitReply {
  job: string;
  unit: SystemdUnit;
}
export { SystemdManagerStopUnitReply };