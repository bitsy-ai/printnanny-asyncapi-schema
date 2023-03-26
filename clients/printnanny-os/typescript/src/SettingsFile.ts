import {SettingsApp} from './SettingsApp';
import {SettingsFormat} from './SettingsFormat';
interface SettingsFile {
  app: SettingsApp;
  content: string;
  file_name: string;
  file_format: SettingsFormat;
}
export { SettingsFile };