import {SettingsFormat} from './SettingsFormat';
interface OctoPrintSettings {
  enabled: boolean;
  install_dir: string;
  settings_file: string;
  settings_format: SettingsFormat;
  venv: string;
}
export { OctoPrintSettings };