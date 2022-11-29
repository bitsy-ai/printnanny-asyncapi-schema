import {SystemdUnit} from './SystemdUnit';
export class SystemdManagerRestartUnitReply {
  private _job: string;
  private _unit: SystemdUnit;

  constructor(input: {
    job: string,
    unit: SystemdUnit,
  }) {
    this._job = input.job;
    this._unit = input.unit;
  }

  get job(): string { return this._job; }
  set job(job: string) { this._job = job; }

  get unit(): SystemdUnit { return this._unit; }
  set unit(unit: SystemdUnit) { this._unit = unit; }
}
