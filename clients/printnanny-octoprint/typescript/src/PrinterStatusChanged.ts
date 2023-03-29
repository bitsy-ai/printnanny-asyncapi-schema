import {Job} from './Job';
import {PrinterStatus} from './PrinterStatus';
interface PrinterStatusChanged {
  job?: Job;
  reserved_status: PrinterStatus;
}
export { PrinterStatusChanged };