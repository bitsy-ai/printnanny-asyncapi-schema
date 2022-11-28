import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnitLoadState} from './SystemdUnitLoadState';
import {SystemdUnitFileState} from './SystemdUnitFileState';
export class SystemdUnit {
  private _id: string;
  private _fragmentPath: string;
  private _activeState: SystemdUnitActiveState;
  private _loadState: SystemdUnitLoadState;
  private _unitFileState: SystemdUnitFileState;

  constructor(input: {
    id: string,
    fragmentPath: string,
    activeState: SystemdUnitActiveState,
    loadState: SystemdUnitLoadState,
    unitFileState: SystemdUnitFileState,
  }) {
    this._id = input.id;
    this._fragmentPath = input.fragmentPath;
    this._activeState = input.activeState;
    this._loadState = input.loadState;
    this._unitFileState = input.unitFileState;
  }

  get id(): string { return this._id; }
  set id(id: string) { this._id = id; }

  get fragmentPath(): string { return this._fragmentPath; }
  set fragmentPath(fragmentPath: string) { this._fragmentPath = fragmentPath; }

  get activeState(): SystemdUnitActiveState { return this._activeState; }
  set activeState(activeState: SystemdUnitActiveState) { this._activeState = activeState; }

  get loadState(): SystemdUnitLoadState { return this._loadState; }
  set loadState(loadState: SystemdUnitLoadState) { this._loadState = loadState; }

  get unitFileState(): SystemdUnitFileState { return this._unitFileState; }
  set unitFileState(unitFileState: SystemdUnitFileState) { this._unitFileState = unitFileState; }
}
