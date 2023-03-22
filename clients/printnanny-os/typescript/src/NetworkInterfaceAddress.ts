
export interface NetworkInterfaceAddress {
  interface_name: string;
  flags: number;
  address?: string;
  netmask?: string;
  broadcast?: string;
  destination?: string;
}
