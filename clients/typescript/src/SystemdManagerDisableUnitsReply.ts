import {SystemdUnitChange} from './SystemdUnitChange';
export class SystemdManagerDisableUnitsReply {
  private _changes: SystemdUnitChange[];

  constructor(input: {
    changes: SystemdUnitChange[],
  }) {
    this._changes = input.changes;
  }

  get changes(): SystemdUnitChange[] { return this._changes; }
  set changes(changes: SystemdUnitChange[]) { this._changes = changes; }
}
