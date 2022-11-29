
export class PrintNannyCloudAuthReply {
  private _statusCode: number;
  private _msg: string;

  constructor(input: {
    statusCode: number,
    msg: string,
  }) {
    this._statusCode = input.statusCode;
    this._msg = input.msg;
  }

  get statusCode(): number { return this._statusCode; }
  set statusCode(statusCode: number) { this._statusCode = statusCode; }

  get msg(): string { return this._msg; }
  set msg(msg: string) { this._msg = msg; }
}
