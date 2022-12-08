import {SettingsFile} from './SettingsFile';
export interface SettingsApplyRequest {
  file: SettingsFile;
  git_head_commit: string;
  git_commit_msg: string;
}
