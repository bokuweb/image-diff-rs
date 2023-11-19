import { readFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { strictEqual } from "node:assert";

import path from "node:path";

import { diff } from "../index.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const imga = await readFile(
  path.resolve(__dirname, "../../fixtures/sample0.webp")
);
const imgb = await readFile(
  path.resolve(__dirname, "../../fixtures/sample1.webp")
);

const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });

strictEqual(result.diffCount, 3454);

console.info("It works.");
