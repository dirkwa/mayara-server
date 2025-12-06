#!/usr/bin/env node
/**
 * Cross-platform build script for mayara-signalk-wasm
 *
 * Usage: node build.js [--test] [--pack]
 *   --test  Run cargo tests before building
 *   --pack  Create npm package after building
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const args = process.argv.slice(2);
const runTests = args.includes('--test');
const runPack = args.includes('--pack');

const WASM_TARGET = 'wasm32-wasip1';
const CRATE_NAME = 'mayara_signalk_wasm';

// Paths (relative to this script's directory)
const scriptDir = __dirname;
const projectRoot = path.resolve(scriptDir, '..');
const wasmSource = path.join(projectRoot, 'target', WASM_TARGET, 'release', `${CRATE_NAME}.wasm`);
const wasmDest = path.join(scriptDir, 'plugin.wasm');

function run(cmd, options = {}) {
  console.log(`> ${cmd}`);
  try {
    execSync(cmd, { stdio: 'inherit', cwd: options.cwd || projectRoot, ...options });
  } catch (e) {
    console.error(`Command failed: ${cmd}`);
    process.exit(1);
  }
}

function main() {
  console.log('=== Mayara SignalK WASM Build ===\n');

  // Step 1: Run tests (optional)
  if (runTests) {
    console.log('Step 1: Running tests...\n');
    run('cargo test -p mayara-core');
    console.log('\n');
  }

  // Step 2: Build WASM
  console.log('Step 2: Building WASM...\n');
  run(`cargo build --target ${WASM_TARGET} --release -p mayara-signalk-wasm`);
  console.log('\n');

  // Step 3: Copy WASM file
  console.log('Step 3: Copying WASM file...\n');
  if (!fs.existsSync(wasmSource)) {
    console.error(`WASM file not found: ${wasmSource}`);
    process.exit(1);
  }
  fs.copyFileSync(wasmSource, wasmDest);
  const size = fs.statSync(wasmDest).size;
  console.log(`Copied ${wasmSource} -> plugin.wasm (${(size / 1024).toFixed(1)} KB)\n`);

  // Step 4: Pack (optional)
  if (runPack) {
    console.log('Step 4: Creating npm package...\n');
    run('npm pack', { cwd: scriptDir });
    console.log('\n');
  }

  console.log('=== Build complete ===');
}

main();
