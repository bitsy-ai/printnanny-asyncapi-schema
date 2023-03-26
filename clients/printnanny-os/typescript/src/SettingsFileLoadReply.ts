import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
interface SettingsFileLoadReply {
  files: SettingsFile[];
  git_head_commit: string;
  git_history: GitCommit[];
}
export { SettingsFileLoadReply };