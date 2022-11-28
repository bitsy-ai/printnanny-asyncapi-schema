
export class SettingsRevertRequest {
  private _gitCommit: string;

  constructor(input: {
    gitCommit: string,
  }) {
    this._gitCommit = input.gitCommit;
  }

  get gitCommit(): string { return this._gitCommit; }
  set gitCommit(gitCommit: string) { this._gitCommit = gitCommit; }
}
