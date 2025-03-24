#!/usr/bin/env node

const fs = require("fs");
const path = require("path");

const dist_dir = path.resolve(__dirname, "dist");
const package_json_path = path.resolve(__dirname, "package.json");
const digest_script = path.resolve(__dirname, "iex-digest-cron");

if (!fs.existsSync(dist_dir)) {
  fs.mkdirSync(dist_dir);
}

// Generate minimal dist package.json
const pkg = JSON.parse(fs.readFileSync(package_json_path, "utf8"));
const dist_package = {
  name: pkg.name,
  version: pkg.version,
  description: pkg.description,
  author: pkg.author,
  license: pkg.license,
  type: pkg.type,
  bin: pkg.bin,
  dependencies: pkg.dependencies,
  scripts: {
    postinstall: pkg.scripts?.postinstall || ""
  }
};

fs.writeFileSync(
  path.join(dist_dir, "package.json"),
  JSON.stringify(dist_package, null, 2),
  "utf8"
);

// Copy README.md if exists
const readme_path = path.resolve(__dirname, "README.md");
if (fs.existsSync(readme_path)) {
  fs.copyFileSync(readme_path, path.join(dist_dir, "README.md"));
  console.log("✓ Copied README.md");
}

// Copy iex-digest-cron and make it executable
if (fs.existsSync(digest_script)) {
  const dest_path = path.join(dist_dir, "iex-digest-cron");
  fs.copyFileSync(digest_script, dest_path);
  fs.chmodSync(dest_path, 0o755);
  console.log("✓ Copied and chmod +x iex-digest-cron");
} else {
  console.warn("⚠️  iex-digest-cron not found in root directory");
}

console.log("✓ Generated dist/package.json successfully.");
