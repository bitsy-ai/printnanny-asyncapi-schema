
export class AnonymousSchema_10 {
  private _files: (string | any)[];

  constructor(input: {
    files: (string | any)[],
  }) {
    this._files = input.files;
  }

  get files(): (string | any)[] { return this._files; }
  set files(files: (string | any)[]) { this._files = files; }
}
