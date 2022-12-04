import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export interface SettingsApplyRequest {
  app: SettingsApp;
  files: SettingsFile[];
  gitHeadCommit: string;
  gitCommitMsg: string;
}
