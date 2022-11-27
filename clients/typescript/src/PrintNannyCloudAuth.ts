
export class PrintNannyCloudAuth {
  private _email: string;
  private _apiToken: string;
  private _apiUrl: string;

  constructor(input: {
    email: string,
    apiToken: string,
    apiUrl: string,
  }) {
    this._email = input.email;
    this._apiToken = input.apiToken;
    this._apiUrl = input.apiUrl;
  }

  get email(): string { return this._email; }
  set email(email: string) { this._email = email; }

  get apiToken(): string { return this._apiToken; }
  set apiToken(apiToken: string) { this._apiToken = apiToken; }

  get apiUrl(): string { return this._apiUrl; }
  set apiUrl(apiUrl: string) { this._apiUrl = apiUrl; }
}
