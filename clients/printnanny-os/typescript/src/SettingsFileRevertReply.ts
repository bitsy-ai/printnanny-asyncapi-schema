import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
interface SettingsFileRevertReply {
  app: SettingsApp;
  files: SettingsFile[];
  git_head_commit: string;
  git_history: GitCommit[];
}
export { SettingsFileRevertReply };