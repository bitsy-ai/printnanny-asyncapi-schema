import {SettingsFormat} from './SettingsFormat';
interface MoonrakerSettings {
  enabled: boolean;
  install_dir: string;
  settings_file: string;
  settings_format: SettingsFormat;
  venv: string;
}
export { MoonrakerSettings };