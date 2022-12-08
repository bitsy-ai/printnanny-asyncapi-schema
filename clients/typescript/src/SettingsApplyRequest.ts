import {SettingsFile} from './SettingsFile';
export interface SettingsApplyRequest {
  files: SettingsFile[];
  git_head_commit: string;
  git_commit_msg: string;
}
