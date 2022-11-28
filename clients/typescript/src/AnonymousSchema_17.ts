
export class AnonymousSchema_17 {
  private _reservedName: string;

  constructor(input: {
    reservedName: string,
  }) {
    this._reservedName = input.reservedName;
  }

  get reservedName(): string { return this._reservedName; }
  set reservedName(reservedName: string) { this._reservedName = reservedName; }
}
