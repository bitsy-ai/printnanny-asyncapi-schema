import {SettingsFile} from './SettingsFile';
export interface SettingsFileApplyRequest {
  file: SettingsFile;
  git_head_commit: string;
  git_commit_msg: string;
}
