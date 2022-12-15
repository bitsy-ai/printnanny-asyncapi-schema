import * as path from 'path';
import * as fs from 'fs';
import { RustFileGenerator, RustRenderCompleteModelOptions, RUST_COMMON_PRESET, defaultRustRenderCompleteModelOptions, RustPackageFeatures, AsyncAPIInputProcessor } from '@asyncapi/modelina';
import YAML from 'yaml'

export async function generate(): Promise<void> {
  // initialize the generator from a preset
  const generator = new RustFileGenerator({
    presets: [
      {
        preset: RUST_COMMON_PRESET, options: {
          implementNew: true,
          implementDefault: true
        }
      }
    ]
  });
  // Generated files will be written to rust/ directory
  const outDir = path.join(__dirname, 'rust');
  const data = fs.readFileSync('../2.4.0/printnanny-os.yml').toString();
  const doc = YAML.parse(data)

  // Run the file generator with options
  const models = await generator.generateToPackage(doc, outDir, {
    ...defaultRustRenderCompleteModelOptions,
    supportFiles: true, // generate Cargo.toml and lib.rs
    package: {
      packageName: 'printnanny-asyncapi-models',
      packageVersion: '0.1.37',
      // set authors, homepage, repository, and license
      authors: ['Leigh Johnson <leigh@printnanny.ai>'],
      homepage: 'https://github.com/bitsy-ai/printnanny-asyncapi-schema',
      repository: 'https://github.com/bitsy-ai/printnanny-asyncapi-schema',
      license: 'Apache-2.0',
      description: 'PritnNanny AsyncAPI Rust models generated by Modelina',
      // support 2018 editions and up
      edition: '2018',
      // enable serde_json
      packageFeatures: [RustPackageFeatures.json] as RustPackageFeatures[]
    }
  } as RustRenderCompleteModelOptions);
  for (const model of models) {
    console.log(model.result);
  }
}
if (require.main === module) {
  generate();
}