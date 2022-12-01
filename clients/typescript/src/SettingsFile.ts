import {SettingsFormat} from './SettingsFormat';
export class SettingsFile {
  private _content: string;
  private _fileName: string;
  private _fileFormat: SettingsFormat;

  constructor(input: {
    content: string,
    fileName: string,
    fileFormat: SettingsFormat,
  }) {
    this._content = input.content;
    this._fileName = input.fileName;
    this._fileFormat = input.fileFormat;
  }

  get content(): string { return this._content; }
  set content(content: string) { this._content = content; }

  get fileName(): string { return this._fileName; }
  set fileName(fileName: string) { this._fileName = fileName; }

  get fileFormat(): SettingsFormat { return this._fileFormat; }
  set fileFormat(fileFormat: SettingsFormat) { this._fileFormat = fileFormat; }
}
