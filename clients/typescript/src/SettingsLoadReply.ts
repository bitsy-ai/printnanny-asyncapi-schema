import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export interface SettingsLoadReply {
  files: SettingsFile[];
  git_head_commit: string;
  git_history: GitCommit[];
}
