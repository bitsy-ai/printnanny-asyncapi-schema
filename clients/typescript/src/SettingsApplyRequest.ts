import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
export class SettingsApplyRequest {
  private _format: SettingsFormat;
  private _filename: SettingsFile;
  private _content: string;
  private _headGitCommit: string;

  constructor(input: {
    format: SettingsFormat,
    filename: SettingsFile,
    content: string,
    headGitCommit: string,
  }) {
    this._format = input.format;
    this._filename = input.filename;
    this._content = input.content;
    this._headGitCommit = input.headGitCommit;
  }

  get format(): SettingsFormat { return this._format; }
  set format(format: SettingsFormat) { this._format = format; }

  get filename(): SettingsFile { return this._filename; }
  set filename(filename: SettingsFile) { this._filename = filename; }

  get content(): string { return this._content; }
  set content(content: string) { this._content = content; }

  get headGitCommit(): string { return this._headGitCommit; }
  set headGitCommit(headGitCommit: string) { this._headGitCommit = headGitCommit; }
}
