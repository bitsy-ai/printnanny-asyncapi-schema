import {SettingsFormat} from './SettingsFormat';
export interface OctoPrintSettings {
  enabled: boolean;
  install_dir: string;
  settings_file: string;
  settings_format: SettingsFormat;
  venv: string;
}
