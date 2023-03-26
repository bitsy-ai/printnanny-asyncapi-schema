import {SystemdUnit} from './SystemdUnit';
interface SystemdManagerRestartUnitReply {
  job: string;
  unit: SystemdUnit;
}
export { SystemdManagerRestartUnitReply };