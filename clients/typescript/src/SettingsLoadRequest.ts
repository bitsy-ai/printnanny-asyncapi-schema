import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
export class SettingsLoadRequest {
  private _format: SettingsFormat;
  private _filename: SettingsFile;

  constructor(input: {
    format: SettingsFormat,
    filename: SettingsFile,
  }) {
    this._format = input.format;
    this._filename = input.filename;
  }

  get format(): SettingsFormat { return this._format; }
  set format(format: SettingsFormat) { this._format = format; }

  get filename(): SettingsFile { return this._filename; }
  set filename(filename: SettingsFile) { this._filename = filename; }
}
