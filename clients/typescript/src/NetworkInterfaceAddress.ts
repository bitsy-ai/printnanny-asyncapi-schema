
export interface NetworkInterfaceAddress {
  interface_name: string;
  flags: string[];
  address?: string;
  netmask?: string;
  broadcast?: string;
  destination?: string;
}
