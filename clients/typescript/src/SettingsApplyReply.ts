import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
export class SettingsApplyReply {
  private _format?: SettingsFormat;
  private _filename?: SettingsFile;
  private _content?: string;
  private _headGitCommit?: string;

  constructor(input: {
    format?: SettingsFormat,
    filename?: SettingsFile,
    content?: string,
    headGitCommit?: string,
  }) {
    this._format = input.format;
    this._filename = input.filename;
    this._content = input.content;
    this._headGitCommit = input.headGitCommit;
  }

  get format(): SettingsFormat | undefined { return this._format; }
  set format(format: SettingsFormat | undefined) { this._format = format; }

  get filename(): SettingsFile | undefined { return this._filename; }
  set filename(filename: SettingsFile | undefined) { this._filename = filename; }

  get content(): string | undefined { return this._content; }
  set content(content: string | undefined) { this._content = content; }

  get headGitCommit(): string | undefined { return this._headGitCommit; }
  set headGitCommit(headGitCommit: string | undefined) { this._headGitCommit = headGitCommit; }
}
