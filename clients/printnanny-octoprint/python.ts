import * as path from 'path';
import * as fs from 'fs';

import { PythonFileGenerator, PYTHON_PYDANTIC_PRESET } from "@asyncapi/modelina";

import YAML from 'yaml'

export async function generate(): Promise<void> {
  // initialize the generator from a preset
  const generator = new PythonFileGenerator({
    presets: [PYTHON_PYDANTIC_PRESET]
  });
  // Generated files will be written to rust/ directory
  const outDir = path.join(__dirname, 'python/src');
  const data = fs.readFileSync('../../2.4.0/printnanny-octoprint.yml').toString();
  const doc = YAML.parse(data)


  // Run the file generator with options
  const models = await generator.generateToFiles(doc, outDir, {});
  for (const model of models) {
    console.log(model.result);
  }
}
if (require.main === module) {
  generate();
}