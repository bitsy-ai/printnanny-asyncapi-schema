import {AnonymousSchema_17} from './AnonymousSchema_17';
export class ActionReply {
  private _request: AnonymousSchema_17;
  private _statusCode: number;
  private _msg: string;

  constructor(input: {
    request: AnonymousSchema_17,
    statusCode: number,
    msg: string,
  }) {
    this._request = input.request;
    this._statusCode = input.statusCode;
    this._msg = input.msg;
  }

  get request(): AnonymousSchema_17 { return this._request; }
  set request(request: AnonymousSchema_17) { this._request = request; }

  get statusCode(): number { return this._statusCode; }
  set statusCode(statusCode: number) { this._statusCode = statusCode; }

  get msg(): string { return this._msg; }
  set msg(msg: string) { this._msg = msg; }
}
