
export interface OctoPrintJobProgress {
  completion?: number;
  filepos?: number;
  printTime?: number;
  printTimeLeft?: number;
  printTimeLeftOrigin?: string;
}
