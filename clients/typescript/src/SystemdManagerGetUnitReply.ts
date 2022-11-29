import {SystemdUnit} from './SystemdUnit';
export class SystemdManagerGetUnitReply {
  private _unit: SystemdUnit;

  constructor(input: {
    unit: SystemdUnit,
  }) {
    this._unit = input.unit;
  }

  get unit(): SystemdUnit { return this._unit; }
  set unit(unit: SystemdUnit) { this._unit = unit; }
}
