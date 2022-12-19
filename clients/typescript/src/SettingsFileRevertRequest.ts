import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export interface SettingsFileRevertRequest {
  app: SettingsApp;
  files: SettingsFile[];
  git_commit: string;
}
