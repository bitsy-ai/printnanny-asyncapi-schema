
enum SystemdUnitFileState {
  ENABLED = "enabled",
  ENABLED_MINUS_RUNTIME = "enabled-runtime",
  LINKED = "linked",
  LINKED_MINUS_RUNTIME = "linked-runtime",
  MASKED = "masked",
  MASKED_MINUS_RUNTIME = "masked-runtime",
  RESERVED_STATIC = "static",
  DISABLED = "disabled",
  INVALID = "invalid",
}
export { SystemdUnitFileState };