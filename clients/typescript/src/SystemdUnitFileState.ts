
export enum SystemdUnitFileState {
  ENABLED = "enabled",
  ENABLED_MINUS_RUNTIME = "enabled-runtime",
  LINKED = "linked",
  LINKED_MINUS_RUNTIME = "linked-runtime",
  RESERVED_STATIC = "static",
  DISABLED = "disabled",
  INVALID = "invalid",
}
