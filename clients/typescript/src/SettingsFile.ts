import {SettingsFormat} from './SettingsFormat';
export interface SettingsFile {
  content: string;
  fileName: string;
  fileFormat: SettingsFormat;
}
