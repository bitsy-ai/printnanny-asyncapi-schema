import {SystemdManagerGetUnitRequest} from './SystemdManagerGetUnitRequest';
import {SystemdUnit} from './SystemdUnit';
export class SystemdManagerGetUnitReply {
  private _request: SystemdManagerGetUnitRequest;
  private _unit: SystemdUnit;

  constructor(input: {
    request: SystemdManagerGetUnitRequest,
    unit: SystemdUnit,
  }) {
    this._request = input.request;
    this._unit = input.unit;
  }

  get request(): SystemdManagerGetUnitRequest { return this._request; }
  set request(request: SystemdManagerGetUnitRequest) { this._request = request; }

  get unit(): SystemdUnit { return this._unit; }
  set unit(unit: SystemdUnit) { this._unit = unit; }
}
