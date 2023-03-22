import { TypeScriptFileGenerator, typeScriptDefaultPropertyKeyConstraints } from '@asyncapi/modelina';
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
const data = fs.readFileSync('../../2.4.0/printnanny-octoprint.yml').toString();
const doc = YAML.parse(data)


export async function generate(): Promise<void> {
  const models = await generator.generateToFiles(doc, "typescript/src", {
    exportType: "named",
  });
  let indexContent = "";
  for (const model of models) {
    indexContent += `export * from "./${model.modelName}";`;
    indexContent += "\n";
    console.log(model.result);
  }
  fs.writeFileSync("typescript/src/index.ts", indexContent);
}
generate();
