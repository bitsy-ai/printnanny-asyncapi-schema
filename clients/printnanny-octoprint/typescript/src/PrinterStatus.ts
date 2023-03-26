
enum PrinterStatus {
  CLOSED_WITH_ERROR = "CLOSED_WITH_ERROR",
  RESERVED_CLOSED = "CLOSED",
  CONNECTING = "CONNECTING",
  DETECT_BAUDRATE = "DETECT_BAUDRATE",
  DETECT_SERIAL = "DETECT_SERIAL",
  ERROR = "ERROR",
  NONE = "NONE",
  OFFLINE = "OFFLINE",
  OPEN_SERIAL = "OPEN_SERIAL",
  OPERATIONAL = "OPERATIONAL",
  PAUSED = "PAUSED",
  PRINTING = "PRINTING",
  TRANSFERING_FILE = "TRANSFERING_FILE",
  UNKNOWN = "UNKNOWN",
}
export { PrinterStatus };