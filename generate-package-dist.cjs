const fs = require('fs');

// Read the original package.json
const packageJson = JSON.parse(fs.readFileSync('./package.json', 'utf8'));

// Filter out unnecessary fields for distribution
const distPackageJson = {
  name: packageJson.name,
  version: packageJson.version,
  description: packageJson.description,
  author: packageJson.author,
  bin: packageJson.bin,
  dependencies: packageJson.dependencies,
  license: packageJson.license,
  scripts: {
    postinstall: packageJson.scripts?.postinstall || ''
  }
};
fs.writeFileSync('./dist/package.json', JSON.stringify(distPackageJson, null, 2));
console.log('Generated package.dist successfully.');
