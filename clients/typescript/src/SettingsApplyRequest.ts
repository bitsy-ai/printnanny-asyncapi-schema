import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export interface SettingsApplyRequest {
  app: SettingsApp;
  files: SettingsFile[];
  git_head_commit: string;
  git_commit_msg: string;
}
