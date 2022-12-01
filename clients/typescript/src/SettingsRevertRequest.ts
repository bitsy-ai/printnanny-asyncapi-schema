import {SettingsApp} from './SettingsApp';
export class SettingsRevertRequest {
  private _app: SettingsApp;
  private _files: any;
  private _gitCommit: string;

  constructor(input: {
    app: SettingsApp,
    files: any,
    gitCommit: string,
  }) {
    this._app = input.app;
    this._files = input.files;
    this._gitCommit = input.gitCommit;
  }

  get app(): SettingsApp { return this._app; }
  set app(app: SettingsApp) { this._app = app; }

  get files(): any { return this._files; }
  set files(files: any) { this._files = files; }

  get gitCommit(): string { return this._gitCommit; }
  set gitCommit(gitCommit: string) { this._gitCommit = gitCommit; }
}
