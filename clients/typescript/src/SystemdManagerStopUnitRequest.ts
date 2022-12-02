
export class SystemdManagerStopUnitRequest {
  private _unitName?: string;

  constructor(input: {
    unitName?: string,
  }) {
    this._unitName = input.unitName;
  }

  get unitName(): string | undefined { return this._unitName; }
  set unitName(unitName: string | undefined) { this._unitName = unitName; }
}
