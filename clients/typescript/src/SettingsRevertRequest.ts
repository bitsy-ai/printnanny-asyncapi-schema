
export class SettingsRevertRequest {
  private _gitCommit?: string;

  constructor(input: {
    gitCommit?: string,
  }) {
    this._gitCommit = input.gitCommit;
  }

  get gitCommit(): string | undefined { return this._gitCommit; }
  set gitCommit(gitCommit: string | undefined) { this._gitCommit = gitCommit; }
}
