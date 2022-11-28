import {SystemdManagerChangeUnitRequest} from './SystemdManagerChangeUnitRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerChangeUnitReply {
  private _request: SystemdManagerChangeUnitRequest;
  private _changes: (SystemdUnitChange | any)[];

  constructor(input: {
    request: SystemdManagerChangeUnitRequest,
    changes: (SystemdUnitChange | any)[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): SystemdManagerChangeUnitRequest { return this._request; }
  set request(request: SystemdManagerChangeUnitRequest) { this._request = request; }

  get changes(): (SystemdUnitChange | any)[] { return this._changes; }
  set changes(changes: (SystemdUnitChange | any)[]) { this._changes = changes; }
}
