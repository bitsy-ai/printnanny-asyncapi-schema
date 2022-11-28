import {SystemdManagerEnableUnitsRequest} from './SystemdManagerEnableUnitsRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerEnableUnitsReply {
  private _request: SystemdManagerEnableUnitsRequest;
  private _changes: SystemdUnitChange[];

  constructor(input: {
    request: SystemdManagerEnableUnitsRequest,
    changes: SystemdUnitChange[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): SystemdManagerEnableUnitsRequest { return this._request; }
  set request(request: SystemdManagerEnableUnitsRequest) { this._request = request; }

  get changes(): SystemdUnitChange[] { return this._changes; }
  set changes(changes: SystemdUnitChange[]) { this._changes = changes; }
}
