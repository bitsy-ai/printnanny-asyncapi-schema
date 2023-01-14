import {SettingsFormat} from './SettingsFormat';
export interface KlipperSettings {
  enabled: boolean;
  install_dir: string;
  settings_file: string;
  settings_format: SettingsFormat;
  venv: string;
}
