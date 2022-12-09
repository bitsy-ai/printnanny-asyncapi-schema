
export interface DeviceInfoLoadReply {
  issue: string;
  os_release: string;
  printnanny_cli_version: string;
  tailscale_address_ipv4?: string;
  tailscale_address_ipv6?: string;
}
