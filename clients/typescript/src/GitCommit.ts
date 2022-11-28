
export class GitCommit {
  private _oid?: string;
  private _header?: string;
  private _message?: string;
  private _ts?: number;
  private _additionalProperties?: Map<string, any>;

  constructor(input: {
    oid?: string,
    header?: string,
    message?: string,
    ts?: number,
    additionalProperties?: Map<string, any>,
  }) {
    this._oid = input.oid;
    this._header = input.header;
    this._message = input.message;
    this._ts = input.ts;
    this._additionalProperties = input.additionalProperties;
  }

  get oid(): string | undefined { return this._oid; }
  set oid(oid: string | undefined) { this._oid = oid; }

  get header(): string | undefined { return this._header; }
  set header(header: string | undefined) { this._header = header; }

  get message(): string | undefined { return this._message; }
  set message(message: string | undefined) { this._message = message; }

  get ts(): number | undefined { return this._ts; }
  set ts(ts: number | undefined) { this._ts = ts; }

  get additionalProperties(): Map<string, any> | undefined { return this._additionalProperties; }
  set additionalProperties(additionalProperties: Map<string, any> | undefined) { this._additionalProperties = additionalProperties; }
}
