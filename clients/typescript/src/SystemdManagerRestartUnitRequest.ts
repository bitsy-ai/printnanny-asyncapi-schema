
export class SystemdManagerRestartUnitRequest {
  private _unitName: string;

  constructor(input: {
    unitName: string,
  }) {
    this._unitName = input.unitName;
  }

  get unitName(): string { return this._unitName; }
  set unitName(unitName: string) { this._unitName = unitName; }
}
