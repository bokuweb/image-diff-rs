import { readFile, writeFile } from "node:fs/promises";
import { fileURLToPath } from "node:url";
import { strictEqual } from "node:assert";

import path from "node:path";

import { diff } from "../index.mjs";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

{
  const patha = path.resolve(__dirname, "../../fixtures/sample0.webp");
  const imga = await readFile(patha);

  const pathb = path.resolve(__dirname, "../../fixtures/sample1.webp");
  const imgb = await readFile(pathb);

  const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
  await writeFile("./diff0.webp", result.diffImage);

  strictEqual(result.diffCount, 3454);
}

{
  const patha = path.resolve(__dirname, "../../fixtures/sample0.webp");
  const imga = await readFile(patha);

  const pathb = path.resolve(__dirname, "../../fixtures/005a.png");
  const imgb = await readFile(pathb);

  const result = diff(imga, imgb, { enableAntiAlias: true, threshold: 0.01 });
  await writeFile("./diff1.webp", result.diffImage);

  strictEqual(result.diffCount, 383111);
}

console.info("It works.");
