
export class GitCommit {
  private _oid: string;
  private _header: string;
  private _message: string;
  private _ts: number;

  constructor(input: {
    oid: string,
    header: string,
    message: string,
    ts: number,
  }) {
    this._oid = input.oid;
    this._header = input.header;
    this._message = input.message;
    this._ts = input.ts;
  }

  get oid(): string { return this._oid; }
  set oid(oid: string) { this._oid = oid; }

  get header(): string { return this._header; }
  set header(header: string) { this._header = header; }

  get message(): string { return this._message; }
  set message(message: string) { this._message = message; }

  get ts(): number { return this._ts; }
  set ts(ts: number) { this._ts = ts; }
}
