import {NetworkInterfaceAddress} from './NetworkInterfaceAddress';
export interface DeviceInfoLoadReply {
  issue: string;
  os_release: string;
  printnanny_cli_version: string;
  ifaddrs: NetworkInterfaceAddress[];
}
