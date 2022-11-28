import {SystemdUnitActiveState} from './SystemdUnitActiveState';
import {SystemdUnitLoadState} from './SystemdUnitLoadState';
import {SystemdUnitFileState} from './SystemdUnitFileState';
export class SystemdUnit {
  private _id?: string;
  private _fragmentPath?: string;
  private _activeState?: SystemdUnitActiveState;
  private _loadState?: SystemdUnitLoadState;
  private _unitFileState?: SystemdUnitFileState;

  constructor(input: {
    id?: string,
    fragmentPath?: string,
    activeState?: SystemdUnitActiveState,
    loadState?: SystemdUnitLoadState,
    unitFileState?: SystemdUnitFileState,
  }) {
    this._id = input.id;
    this._fragmentPath = input.fragmentPath;
    this._activeState = input.activeState;
    this._loadState = input.loadState;
    this._unitFileState = input.unitFileState;
  }

  get id(): string | undefined { return this._id; }
  set id(id: string | undefined) { this._id = id; }

  get fragmentPath(): string | undefined { return this._fragmentPath; }
  set fragmentPath(fragmentPath: string | undefined) { this._fragmentPath = fragmentPath; }

  get activeState(): SystemdUnitActiveState | undefined { return this._activeState; }
  set activeState(activeState: SystemdUnitActiveState | undefined) { this._activeState = activeState; }

  get loadState(): SystemdUnitLoadState | undefined { return this._loadState; }
  set loadState(loadState: SystemdUnitLoadState | undefined) { this._loadState = loadState; }

  get unitFileState(): SystemdUnitFileState | undefined { return this._unitFileState; }
  set unitFileState(unitFileState: SystemdUnitFileState | undefined) { this._unitFileState = unitFileState; }
}
