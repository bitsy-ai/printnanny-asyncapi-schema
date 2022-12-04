import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export interface SettingsRevertRequest {
  app: SettingsApp;
  files: SettingsFile[];
  git_commit: string;
}
