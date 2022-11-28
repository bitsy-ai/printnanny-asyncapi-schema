import {SystemdUnit} from './SystemdUnit';
export class AnonymousSchema_19 {
  private _request: any;
  private _unit: SystemdUnit;

  constructor(input: {
    request: any,
    unit: SystemdUnit,
  }) {
    this._request = input.request;
    this._unit = input.unit;
  }

  get request(): any { return this._request; }
  set request(request: any) { this._request = request; }

  get unit(): SystemdUnit { return this._unit; }
  set unit(unit: SystemdUnit) { this._unit = unit; }
}
