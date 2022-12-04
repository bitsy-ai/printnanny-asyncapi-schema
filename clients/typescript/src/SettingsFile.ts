import {SettingsFormat} from './SettingsFormat';
export interface SettingsFile {
  content: string;
  file_name: string;
  file_format: SettingsFormat;
}
