import {AnonymousSchema_12} from './AnonymousSchema_12';
import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerChangeUnitReply {
  private _request: AnonymousSchema_12;
  private _changes: (SystemdUnitChange | any)[];

  constructor(input: {
    request: AnonymousSchema_12,
    changes: (SystemdUnitChange | any)[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): AnonymousSchema_12 { return this._request; }
  set request(request: AnonymousSchema_12) { this._request = request; }

  get changes(): (SystemdUnitChange | any)[] { return this._changes; }
  set changes(changes: (SystemdUnitChange | any)[]) { this._changes = changes; }
}
