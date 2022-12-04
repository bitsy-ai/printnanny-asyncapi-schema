import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export interface SettingsLoadReply {
  app: SettingsApp;
  files: SettingsFile[];
  gitHeadCommit: string;
  gitHistory: GitCommit[];
}
