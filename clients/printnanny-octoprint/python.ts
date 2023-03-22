import * as path from 'path';
import * as fs from 'fs';

import { PythonFileGenerator, PYTHON_PYDANTIC_PRESET, PYTHON_JSON_SERIALIZER_PRESET } from "@asyncapi/modelina";

import YAML from 'yaml'

export async function generate(): Promise<void> {
  // initialize the generator from a preset
  const generator = new PythonFileGenerator({
    presets: [PYTHON_PYDANTIC_PRESET, PYTHON_JSON_SERIALIZER_PRESET]
  });
  // Generated files will be written to rust/ directory
  const outDir = path.join(__dirname, 'python/printnanny_octoprint_models');
  const data = fs.readFileSync('../../2.4.0/printnanny-octoprint.yml').toString();
  const doc = YAML.parse(data)


  // Run the file generator with options
  let initModule = "";
  const models = await generator.generateToFiles(doc, outDir, { importsStyle: 'explicit' });
  for (const model of models) {
    initModule += `from .${model.modelName} import *`;
    initModule += "\n";
    console.log(model);
  }
  console.log(initModule)
  fs.writeFileSync("python/printnanny_octoprint_models/__init__.py", initModule)
}
if (require.main === module) {
  generate();
}