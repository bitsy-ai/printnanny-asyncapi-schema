import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export interface SettingsFileApplyReply {
  file: SettingsFile;
  git_head_commit: string;
  git_history: GitCommit[];
}
