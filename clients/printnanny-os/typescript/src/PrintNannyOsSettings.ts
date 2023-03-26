import {PrintNannyCloudApiConfig} from './PrintNannyCloudApiConfig';
import {VideoStreamSettings} from './VideoStreamSettings';
import {GitSettings} from './GitSettings';
import {PathSettings} from './PathSettings';
import {KlipperSettings} from './KlipperSettings';
import {MainsailSettings} from './MainsailSettings';
import {MoonrakerSettings} from './MoonrakerSettings';
import {OctoPrintSettings} from './OctoPrintSettings';
interface PrintNannyOsSettings {
  cloud: PrintNannyCloudApiConfig;
  video: VideoStreamSettings;
  git: GitSettings;
  paths: PathSettings;
  klipper: KlipperSettings;
  mainsail: MainsailSettings;
  moonraker: MoonrakerSettings;
  octoprint: OctoPrintSettings;
}
export { PrintNannyOsSettings };