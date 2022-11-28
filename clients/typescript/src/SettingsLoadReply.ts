import {SettingsFormat} from './SettingsFormat';
import {SettingsFile} from './SettingsFile';
import {GitCommit} from './GitCommit';
export class SettingsLoadReply {
  private _content?: string;
  private _format?: SettingsFormat;
  private _filename?: SettingsFile;
  private _gitHistory?: (GitCommit | any)[];
  private _headGitCommit?: string;

  constructor(input: {
    content?: string,
    format?: SettingsFormat,
    filename?: SettingsFile,
    gitHistory?: (GitCommit | any)[],
    headGitCommit?: string,
  }) {
    this._content = input.content;
    this._format = input.format;
    this._filename = input.filename;
    this._gitHistory = input.gitHistory;
    this._headGitCommit = input.headGitCommit;
  }

  get content(): string | undefined { return this._content; }
  set content(content: string | undefined) { this._content = content; }

  get format(): SettingsFormat | undefined { return this._format; }
  set format(format: SettingsFormat | undefined) { this._format = format; }

  get filename(): SettingsFile | undefined { return this._filename; }
  set filename(filename: SettingsFile | undefined) { this._filename = filename; }

  get gitHistory(): (GitCommit | any)[] | undefined { return this._gitHistory; }
  set gitHistory(gitHistory: (GitCommit | any)[] | undefined) { this._gitHistory = gitHistory; }

  get headGitCommit(): string | undefined { return this._headGitCommit; }
  set headGitCommit(headGitCommit: string | undefined) { this._headGitCommit = headGitCommit; }
}
