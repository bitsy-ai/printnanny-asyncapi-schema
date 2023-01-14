import {SettingsFormat} from './SettingsFormat';
export interface MoonrakerSettings {
  enabled: boolean;
  install_dir: string;
  settings_file: string;
  settings_format: SettingsFormat;
  venv: string;
}
