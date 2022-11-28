
export class AnonymousSchema_14 {
  private _additionalProperties?: Map<string, any>;

  constructor(input: {
    additionalProperties?: Map<string, any>,
  }) {
    this._additionalProperties = input.additionalProperties;
  }

  get additionalProperties(): Map<string, any> | undefined { return this._additionalProperties; }
  set additionalProperties(additionalProperties: Map<string, any> | undefined) { this._additionalProperties = additionalProperties; }
}
