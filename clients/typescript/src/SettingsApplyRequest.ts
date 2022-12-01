import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export class SettingsApplyRequest {
  private _app: SettingsApp;
  private _files: SettingsFile[];
  private _gitHeadCommit: string;
  private _gitCommitMsg: string;

  constructor(input: {
    app: SettingsApp,
    files: SettingsFile[],
    gitHeadCommit: string,
    gitCommitMsg: string,
  }) {
    this._app = input.app;
    this._files = input.files;
    this._gitHeadCommit = input.gitHeadCommit;
    this._gitCommitMsg = input.gitCommitMsg;
  }

  get app(): SettingsApp { return this._app; }
  set app(app: SettingsApp) { this._app = app; }

  get files(): SettingsFile[] { return this._files; }
  set files(files: SettingsFile[]) { this._files = files; }

  get gitHeadCommit(): string { return this._gitHeadCommit; }
  set gitHeadCommit(gitHeadCommit: string) { this._gitHeadCommit = gitHeadCommit; }

  get gitCommitMsg(): string { return this._gitCommitMsg; }
  set gitCommitMsg(gitCommitMsg: string) { this._gitCommitMsg = gitCommitMsg; }
}
