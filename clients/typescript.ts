import { TypeScriptFileGenerator, TS_COMMON_PRESET, TS_DEFAULT_PRESET, typeScriptDefaultModelNameConstraints, typeScriptDefaultPropertyKeyConstraints } from '@asyncapi/modelina';
import * as path from 'path';
import * as fs from 'fs';
import YAML from 'yaml'

const generator = new TypeScriptFileGenerator({
  renderTypes: true,
  modelType: "interface",
  constraints: {
    propertyKey: typeScriptDefaultPropertyKeyConstraints({
      NAMING_FORMATTER: (name) => {
        return name;
      }
    })
  }
});

// Generated files will be written to typescript/src directory
const data = fs.readFileSync('../2.4.0/printnanny-os.yml').toString();
const doc = YAML.parse(data)


export async function generate(): Promise<void> {
  const models = await generator.generateToFiles(doc, "typescript/src", {
    exportType: "named",
  });
  for (const model of models) {
    console.log(model.result);
  }
}
generate();
