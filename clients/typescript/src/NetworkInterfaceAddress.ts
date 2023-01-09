
export interface NetworkInterfaceAddress {
  interface_name: string;
  flags: string[];
  address?: string;
  network?: string;
  broadcast?: string;
  destination?: string;
}
