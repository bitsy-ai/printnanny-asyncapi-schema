import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export class SettingsReply {
  private _format?: SettingsFormat;
  private _filename?: SettingsFile;
  private _content?: string;
  private _headGitCommit?: string;
  private _gitHistory?: GitCommit[];

  constructor(input: {
    format?: SettingsFormat,
    filename?: SettingsFile,
    content?: string,
    headGitCommit?: string,
    gitHistory?: GitCommit[],
  }) {
    this._format = input.format;
    this._filename = input.filename;
    this._content = input.content;
    this._headGitCommit = input.headGitCommit;
    this._gitHistory = input.gitHistory;
  }

  get format(): SettingsFormat | undefined { return this._format; }
  set format(format: SettingsFormat | undefined) { this._format = format; }

  get filename(): SettingsFile | undefined { return this._filename; }
  set filename(filename: SettingsFile | undefined) { this._filename = filename; }

  get content(): string | undefined { return this._content; }
  set content(content: string | undefined) { this._content = content; }

  get headGitCommit(): string | undefined { return this._headGitCommit; }
  set headGitCommit(headGitCommit: string | undefined) { this._headGitCommit = headGitCommit; }

  get gitHistory(): GitCommit[] | undefined { return this._gitHistory; }
  set gitHistory(gitHistory: GitCommit[] | undefined) { this._gitHistory = gitHistory; }
}
