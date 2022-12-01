import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export class SettingsLoadReply {
  private _format: SettingsFormat;
  private _filename: SettingsFile;
  private _content: string;
  private _gitHeadCommit: string;
  private _gitHistory: GitCommit[];

  constructor(input: {
    format: SettingsFormat,
    filename: SettingsFile,
    content: string,
    gitHeadCommit: string,
    gitHistory: GitCommit[],
  }) {
    this._format = input.format;
    this._filename = input.filename;
    this._content = input.content;
    this._gitHeadCommit = input.gitHeadCommit;
    this._gitHistory = input.gitHistory;
  }

  get format(): SettingsFormat { return this._format; }
  set format(format: SettingsFormat) { this._format = format; }

  get filename(): SettingsFile { return this._filename; }
  set filename(filename: SettingsFile) { this._filename = filename; }

  get content(): string { return this._content; }
  set content(content: string) { this._content = content; }

  get gitHeadCommit(): string { return this._gitHeadCommit; }
  set gitHeadCommit(gitHeadCommit: string) { this._gitHeadCommit = gitHeadCommit; }

  get gitHistory(): GitCommit[] { return this._gitHistory; }
  set gitHistory(gitHistory: GitCommit[]) { this._gitHistory = gitHistory; }
}
