import { performance } from "perf_hooks";
import { TarBuilder } from "tar-wasm";
import TarJs from "tar-js";
import Tarts from "tarts";
import { LoremIpsum } from "lorem-ipsum";

const files: Record<string, Uint8Array> = {};

const BENCH_SIZE = 1001;

function prepare() {
  const lorem = new LoremIpsum({
    sentencesPerParagraph: {
      max: 64,
      min: 4,
    },
    wordsPerSentence: {
      max: 16,
      min: 4,
    },
  });

  const enc = new TextEncoder();

  for (let i = 0; i < BENCH_SIZE; i++) {
    const fileContent = lorem.generateParagraphs(3);
    files[`file-${i}`] = new Uint8Array(enc.encode(fileContent));
  }
}

function runTarWasm() {
  const tar = new TarBuilder();

  for (const [name, content] of Object.entries(files)) {
    tar.add_file(name, content);
  }
  tar.finish();

  return;
}

function runTarJs() {
  const tar = new TarJs();

  for (const [name, content] of Object.entries(files)) {
    tar.append(name, content);
  }

  return tar;
}

function runTarts() {
  const tar = Tarts(
    Object.entries(files).map(([name, content]) => ({ name, content }))
  );
  return tar;
}

function bench() {
  console.log("Running benchmark...");
  const t0 = performance.now();
  prepare();
  const t1 = performance.now();
  console.log(`Generated ${BENCH_SIZE} random files to test in ${t1 - t0}ms`);
  runTarWasm();
  const t2 = performance.now();
  const timeTarWasm = t2 - t1;
  console.log(`tar-wasm: ${timeTarWasm}ms (baseline)`);
  runTarJs();
  const t3 = performance.now();
  const timeTarJs = t3 - t2;
  console.log(
    `tar-js: ${timeTarJs}ms (${timeTarJs / timeTarWasm}x slower than tar-wasm)`
  );
  runTarts();
  const t4 = performance.now();
  const timeTarts = t4 - t3;
  console.log(
    `tarts: ${timeTarts}ms (${timeTarts / timeTarWasm}x slower than tar-wasm)`
  );
}
bench();
