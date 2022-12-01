import {SettingsApp} from './SettingsApp';
import {GitCommit} from './GitCommit';
export class SettingsLoadReply {
  private _app: SettingsApp;
  private _files: any;
  private _gitHeadCommit: string;
  private _gitHistory: GitCommit[];

  constructor(input: {
    app: SettingsApp,
    files: any,
    gitHeadCommit: string,
    gitHistory: GitCommit[],
  }) {
    this._app = input.app;
    this._files = input.files;
    this._gitHeadCommit = input.gitHeadCommit;
    this._gitHistory = input.gitHistory;
  }

  get app(): SettingsApp { return this._app; }
  set app(app: SettingsApp) { this._app = app; }

  get files(): any { return this._files; }
  set files(files: any) { this._files = files; }

  get gitHeadCommit(): string { return this._gitHeadCommit; }
  set gitHeadCommit(gitHeadCommit: string) { this._gitHeadCommit = gitHeadCommit; }

  get gitHistory(): GitCommit[] { return this._gitHistory; }
  set gitHistory(gitHistory: GitCommit[]) { this._gitHistory = gitHistory; }
}
