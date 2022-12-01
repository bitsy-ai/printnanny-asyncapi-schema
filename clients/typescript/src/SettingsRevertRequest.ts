import {SettingsApp} from './SettingsApp';
import {SettingsFile} from './SettingsFile';
export class SettingsRevertRequest {
  private _app: SettingsApp;
  private _files: SettingsFile[];
  private _gitCommit: string;

  constructor(input: {
    app: SettingsApp,
    files: SettingsFile[],
    gitCommit: string,
  }) {
    this._app = input.app;
    this._files = input.files;
    this._gitCommit = input.gitCommit;
  }

  get app(): SettingsApp { return this._app; }
  set app(app: SettingsApp) { this._app = app; }

  get files(): SettingsFile[] { return this._files; }
  set files(files: SettingsFile[]) { this._files = files; }

  get gitCommit(): string { return this._gitCommit; }
  set gitCommit(gitCommit: string) { this._gitCommit = gitCommit; }
}
