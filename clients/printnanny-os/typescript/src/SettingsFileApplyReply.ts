import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
interface SettingsFileApplyReply {
  file: SettingsFile;
  git_head_commit: string;
  git_history: GitCommit[];
}
export { SettingsFileApplyReply };