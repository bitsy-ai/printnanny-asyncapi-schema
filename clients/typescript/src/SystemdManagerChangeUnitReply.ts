import {SystemdManagerChangeUnitRequest} from './SystemdManagerChangeUnitRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerChangeUnitReply {
  private _request: SystemdManagerChangeUnitRequest;
  private _changes: SystemdUnitChange[];

  constructor(input: {
    request: SystemdManagerChangeUnitRequest,
    changes: SystemdUnitChange[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): SystemdManagerChangeUnitRequest { return this._request; }
  set request(request: SystemdManagerChangeUnitRequest) { this._request = request; }

  get changes(): SystemdUnitChange[] { return this._changes; }
  set changes(changes: SystemdUnitChange[]) { this._changes = changes; }
}
