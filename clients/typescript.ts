import { TypeScriptFileGenerator, AsyncAPIInputProcessor } from '@asyncapi/modelina';
import * as path from 'path';
import * as fs from 'fs';
import YAML from 'yaml'

const generator = new TypeScriptFileGenerator();

// Generated files will be written to typescript/src directory
const outDir = path.join(__dirname, 'rust');
const data = fs.readFileSync('../2.4.0/printnanny-os.yml').toString();
const doc = YAML.parse(data)
const processer = new AsyncAPIInputProcessor();
const inputModel = await processer.process(doc);



export async function generate(): Promise<void> {
  const models = await generator.generateToFiles(inputModel, "typescript/src", { exportType: "named" });
  for (const model of models) {
    console.log(model.result);
  }
}
generate();
