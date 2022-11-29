
export class SystemdManagerDisableUnitsRequest {
  private _files: string[];

  constructor(input: {
    files: string[],
  }) {
    this._files = input.files;
  }

  get files(): string[] { return this._files; }
  set files(files: string[]) { this._files = files; }
}
