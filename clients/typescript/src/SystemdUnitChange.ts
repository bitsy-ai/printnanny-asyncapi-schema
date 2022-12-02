import {SystemdUnitChangeState} from './SystemdUnitChangeState';
export class SystemdUnitChange {
  private _change: SystemdUnitChangeState;
  private _file: string;
  private _destination: string;

  constructor(input: {
    change: SystemdUnitChangeState,
    file: string,
    destination: string,
  }) {
    this._change = input.change;
    this._file = input.file;
    this._destination = input.destination;
  }

  get change(): SystemdUnitChangeState { return this._change; }
  set change(change: SystemdUnitChangeState) { this._change = change; }

  get file(): string { return this._file; }
  set file(file: string) { this._file = file; }

  get destination(): string { return this._destination; }
  set destination(destination: string) { this._destination = destination; }
}
