import {SystemdUnit} from './SystemdUnit';
interface SystemdManagerStartUnitReply {
  job: string;
  unit: SystemdUnit;
}
export { SystemdManagerStartUnitReply };