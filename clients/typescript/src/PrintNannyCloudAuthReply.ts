import {PrintNannyCloudAuthRequest} from './PrintNannyCloudAuthRequest';
export class PrintNannyCloudAuthReply {
  private _request: PrintNannyCloudAuthRequest;
  private _statusCode: number;
  private _msg: string;

  constructor(input: {
    request: PrintNannyCloudAuthRequest,
    statusCode: number,
    msg: string,
  }) {
    this._request = input.request;
    this._statusCode = input.statusCode;
    this._msg = input.msg;
  }

  get request(): PrintNannyCloudAuthRequest { return this._request; }
  set request(request: PrintNannyCloudAuthRequest) { this._request = request; }

  get statusCode(): number { return this._statusCode; }
  set statusCode(statusCode: number) { this._statusCode = statusCode; }

  get msg(): string { return this._msg; }
  set msg(msg: string) { this._msg = msg; }
}