import { copyFileSync, existsSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const websiteRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const projectRoot = resolve(websiteRoot, "..");
const source = join(projectRoot, "assets", "THIRD_PARTY_LICENSES.txt");
const destination = join(websiteRoot, "public", "THIRD_PARTY_LICENSES.txt");

if (!existsSync(source)) {
  throw new Error(`Missing generated license bundle: ${source}`);
}

copyFileSync(source, destination);
