import {PrintNannyCameraSettings} from './PrintNannyCameraSettings';
import {PrintNannyCloudApiConfig} from './PrintNannyCloudApiConfig';
import {GitSettings} from './GitSettings';
import {PathSettings} from './PathSettings';
import {KlipperSettings} from './KlipperSettings';
import {MainsailSettings} from './MainsailSettings';
import {MoonrakerSettings} from './MoonrakerSettings';
import {OctoPrintSettings} from './OctoPrintSettings';
export interface PrintNannyOsSettings {
  camera?: PrintNannyCameraSettings;
  cloud?: PrintNannyCloudApiConfig;
  git?: GitSettings;
  paths?: PathSettings;
  klipper?: KlipperSettings;
  mainsail?: MainsailSettings;
  moonraker?: MoonrakerSettings;
  octoprint?: OctoPrintSettings;
}
