import {SystemdManagerDisableUnitsRequest} from './SystemdManagerDisableUnitsRequest';
import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerDisableUnitsReply {
  private _request: SystemdManagerDisableUnitsRequest;
  private _changes: SystemdUnitChange[];

  constructor(input: {
    request: SystemdManagerDisableUnitsRequest,
    changes: SystemdUnitChange[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): SystemdManagerDisableUnitsRequest { return this._request; }
  set request(request: SystemdManagerDisableUnitsRequest) { this._request = request; }

  get changes(): SystemdUnitChange[] { return this._changes; }
  set changes(changes: SystemdUnitChange[]) { this._changes = changes; }
}
