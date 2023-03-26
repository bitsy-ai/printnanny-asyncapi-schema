import {SystemdUnit} from './SystemdUnit';
interface SystemdManagerReloadUnitReply {
  job: string;
  unit: SystemdUnit;
}
export { SystemdManagerReloadUnitReply };