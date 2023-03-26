import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
interface SettingsFileRevertRequest {
  app: SettingsApp;
  files: SettingsFile[];
  git_commit: string;
}
export { SettingsFileRevertRequest };