import {SettingsApp} from './SettingsApp';
export class SettingsLoadRequest {
  private _app: SettingsApp;

  constructor(input: {
    app: SettingsApp,
  }) {
    this._app = input.app;
  }

  get app(): SettingsApp { return this._app; }
  set app(app: SettingsApp) { this._app = app; }
}
