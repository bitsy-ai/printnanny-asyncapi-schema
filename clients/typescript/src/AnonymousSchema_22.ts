import {AnonymousSchema_23} from './AnonymousSchema_23';
export class AnonymousSchema_22 {
  private _request: AnonymousSchema_23;
  private _statusCode: number;
  private _msg: string;

  constructor(input: {
    request: AnonymousSchema_23,
    statusCode: number,
    msg: string,
  }) {
    this._request = input.request;
    this._statusCode = input.statusCode;
    this._msg = input.msg;
  }

  get request(): AnonymousSchema_23 { return this._request; }
  set request(request: AnonymousSchema_23) { this._request = request; }

  get statusCode(): number { return this._statusCode; }
  set statusCode(statusCode: number) { this._statusCode = statusCode; }

  get msg(): string { return this._msg; }
  set msg(msg: string) { this._msg = msg; }
}
