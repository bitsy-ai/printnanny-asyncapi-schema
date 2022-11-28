import {AnonymousSchema_14} from './AnonymousSchema_14';
export class AnonymousSchema_13 {
  private _request: AnonymousSchema_14;
  private _changes: (string | any)[];

  constructor(input: {
    request: AnonymousSchema_14,
    changes: (string | any)[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): AnonymousSchema_14 { return this._request; }
  set request(request: AnonymousSchema_14) { this._request = request; }

  get changes(): (string | any)[] { return this._changes; }
  set changes(changes: (string | any)[]) { this._changes = changes; }
}
