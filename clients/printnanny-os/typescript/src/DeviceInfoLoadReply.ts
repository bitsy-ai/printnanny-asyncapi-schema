import {NetworkInterfaceAddress} from './NetworkInterfaceAddress';
interface DeviceInfoLoadReply {
  issue: string;
  os_release: string;
  printnanny_cli_version: string;
  ifaddrs: NetworkInterfaceAddress[];
}
export { DeviceInfoLoadReply };