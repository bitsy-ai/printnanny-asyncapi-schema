import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
export class SettingsApplyRequest {
  private _format: SettingsFormat;
  private _filename: SettingsFile;
  private _content: string;
  private _gitHeadCommit: string;
  private _gitCommitMsg: string;

  constructor(input: {
    format: SettingsFormat,
    filename: SettingsFile,
    content: string,
    gitHeadCommit: string,
    gitCommitMsg: string,
  }) {
    this._format = input.format;
    this._filename = input.filename;
    this._content = input.content;
    this._gitHeadCommit = input.gitHeadCommit;
    this._gitCommitMsg = input.gitCommitMsg;
  }

  get format(): SettingsFormat { return this._format; }
  set format(format: SettingsFormat) { this._format = format; }

  get filename(): SettingsFile { return this._filename; }
  set filename(filename: SettingsFile) { this._filename = filename; }

  get content(): string { return this._content; }
  set content(content: string) { this._content = content; }

  get gitHeadCommit(): string { return this._gitHeadCommit; }
  set gitHeadCommit(gitHeadCommit: string) { this._gitHeadCommit = gitHeadCommit; }

  get gitCommitMsg(): string { return this._gitCommitMsg; }
  set gitCommitMsg(gitCommitMsg: string) { this._gitCommitMsg = gitCommitMsg; }
}
