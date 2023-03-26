import {SettingsFile} from './SettingsFile';
interface SettingsFileApplyRequest {
  file: SettingsFile;
  git_head_commit: string;
  git_commit_msg: string;
}
export { SettingsFileApplyRequest };