#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$PROJECT_ROOT"

echo "Preparing questions..."
bun website/scripts/prepare-questions.ts

export CARGO_WEB_BUILD=1

echo "Building web crate (profile: web)..."
cargo rustc --package web --target wasm32-unknown-unknown --profile web -- -C link-arg=-zstack-size=8388608

WASM_PATH="$PROJECT_ROOT/target/wasm32-unknown-unknown/web/web.wasm"
if [ ! -f "$WASM_PATH" ]; then
    echo "WASM binary not found at $WASM_PATH" >&2
    exit 1
fi

OUT_DIR="$PROJECT_ROOT/website/public/runtime"
mkdir -p "$OUT_DIR"

TEMP_DIR="$PROJECT_ROOT/target/wasm32-unknown-unknown/web/bindgen_temp"
rm -rf "$TEMP_DIR"
mkdir -p "$TEMP_DIR"

echo "Generating JS bindings with wasm-bindgen..."
wasm-bindgen "$WASM_PATH" --out-dir "$TEMP_DIR" --target web --no-typescript

WASM_BG="$TEMP_DIR/web_bg.wasm"
if command -v wasm-opt >/dev/null 2>&1; then
    echo "Optimizing with wasm-opt -Oz --all-features..."
    wasm-opt -Oz --all-features "$WASM_BG" -o "$WASM_BG"
else
    echo "Warning: wasm-opt not found in PATH; skipping wasm-opt optimization." >&2
fi

HASH=$(sha256sum "$WASM_BG" | head -c 8 | tr '[:upper:]' '[:lower:]')
HASHED_WASM="web_${HASH}_bg.wasm"
HASHED_JS="web_${HASH}.js"

rm -f "$OUT_DIR"/web_*_bg.wasm
rm -f "$OUT_DIR"/web_*.js

cp "$WASM_BG" "$OUT_DIR/$HASHED_WASM"
sed "s/web_bg\\.wasm/$HASHED_WASM/g" "$TEMP_DIR/web.js" > "$OUT_DIR/$HASHED_JS"

VERSION=$(grep -m1 -E '^[[:space:]]*version[[:space:]]*=' "$PROJECT_ROOT/Cargo.toml" | sed -E 's/.*"([^"]+)".*/\1/')
if [ -z "$VERSION" ]; then
    VERSION="1.8.0"
fi
WASM_BYTES=$(wc -c < "$OUT_DIR/$HASHED_WASM" | tr -d ' ')
BUILT_AT=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

cat <<EOF > "$OUT_DIR/build.json"
{
  "version": "$VERSION",
  "js": "$HASHED_JS",
  "wasm": "$HASHED_WASM",
  "wasmBytes": $WASM_BYTES,
  "builtAt": "$BUILT_AT"
}
EOF

# Calculate sizes and check budget
IMAGES_DIR="$OUT_DIR/images"
IMAGES_BYTES=0
if [ -d "$IMAGES_DIR" ]; then
    IMAGES_BYTES=$(du -sb "$IMAGES_DIR" 2>/dev/null | cut -f1 || du -sk "$IMAGES_DIR" | awk '{print $1 * 1024}')
fi
TOTAL_RUNTIME_BYTES=$(du -sb "$OUT_DIR" 2>/dev/null | cut -f1 || du -sk "$OUT_DIR" | awk '{print $1 * 1024}')

IMAGES_MB=$(bun -e "console.log(($IMAGES_BYTES / 1024 / 1024).toFixed(2))")
WASM_MB=$(bun -e "console.log(($WASM_BYTES / 1024 / 1024).toFixed(2))")
TOTAL_MB=$(bun -e "console.log(($TOTAL_RUNTIME_BYTES / 1024 / 1024).toFixed(2))")

echo "Runtime Assets Summary:"
echo "  Wasm binary:          $WASM_BYTES bytes (${WASM_MB} MB)"
echo "  Images ($IMAGES_DIR): $IMAGES_BYTES bytes (${IMAGES_MB} MB)"
echo "  Total runtime assets: $TOTAL_RUNTIME_BYTES bytes (${TOTAL_MB} MB)"

# 25 MB budget = 26214400 bytes
if [ "$TOTAL_RUNTIME_BYTES" -gt 26214400 ]; then
    echo "Error: Web assets size (${TOTAL_MB} MB) exceeds 25 MB budget." >&2
    exit 1
fi

bun -e "const fs = require('node:fs'); const zlib = require('node:zlib'); const file = process.argv.slice(1).find(a => !a.startsWith('-') && fs.existsSync(a)); const buf = fs.readFileSync(file); const raw = buf.length; const gz = zlib.gzipSync(buf, { level: 9 }).length; const br = zlib.brotliCompressSync(buf, { params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 } }).length; console.log('.wasm sizes for ' + file + ':'); console.log('  Raw:    ' + raw.toLocaleString() + ' bytes (' + (raw / 1024 / 1024).toFixed(2) + ' MB)'); console.log('  Gzip:   ' + gz.toLocaleString() + ' bytes (' + (gz / 1024 / 1024).toFixed(2) + ' MB)'); console.log('  Brotli: ' + br.toLocaleString() + ' bytes (' + (br / 1024 / 1024).toFixed(2) + ' MB)');" "$OUT_DIR/$HASHED_WASM"

echo "Web build complete! Artifacts written to $OUT_DIR"