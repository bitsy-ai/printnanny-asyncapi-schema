import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerDisableUnitsReply {
  private _request: any;
  private _changes: SystemdUnitChange[];

  constructor(input: {
    request: any,
    changes: SystemdUnitChange[],
  }) {
    this._request = input.request;
    this._changes = input.changes;
  }

  get request(): any { return this._request; }
  set request(request: any) { this._request = request; }

  get changes(): SystemdUnitChange[] { return this._changes; }
  set changes(changes: SystemdUnitChange[]) { this._changes = changes; }
}
