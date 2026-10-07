import { spawnSync } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { resolve, dirname } from "node:path";

const root = resolve(process.argv[2] ?? ".");
const outPath = resolve(process.argv[3] ?? `${root}/index.html`);

const run = (cmd, args, env = {}) => {
  const r = spawnSync(cmd, args, { cwd: root, stdio: "inherit", shell: true, env: { ...process.env, ...env } });
  if (r.status) process.exit(r.status);
};

run("cargo", ["build", "--release", "--target", "wasm32-unknown-unknown"], {
  RUSTFLAGS: "-C link-arg=-zstack-size=32768 -C link-arg=--initial-memory=65536 -C link-arg=--max-memory=65536",
});

const raw = `${root}/target/wasm32-unknown-unknown/release/ouzel.wasm`;
const opt = `${root}/target/ouzel.opt.wasm`;
run("wasm-opt", [
  `"${raw}"`, "-O3", "--converge", "--strip-debug", "--strip-producers", "--strip-target-features",
  "--enable-bulk-memory", "--enable-sign-ext", "--enable-mutable-globals",
  "--enable-nontrapping-float-to-int", "--enable-multivalue", "--enable-reference-types",
  "-o", `"${opt}"`,
]);
function scrub(buf) {
  const rd = (o) => { let v = 0, s = 0, b; do { b = buf[o++]; v |= (b & 127) << s; s += 7; } while (b & 128); return [v, o]; };
  const wr = (v) => { const a = []; do { let b = v & 127; v >>>= 7; if (v) b |= 128; a.push(b); } while (v); return a; };
  const rename = { memory: "21" };
  const out = [...buf.subarray(0, 8)];
  let o = 8;
  while (o < buf.length) {
    const id = buf[o];
    const [size, p] = rd(o + 1);
    const body = buf.subarray(p, p + size);
    o = p + size;
    if (id !== 7) { out.push(id, ...wr(size), ...body); continue; }
    let q = 0;
    const rb = (at) => { let v = 0, s = 0, b; do { b = body[at++]; v |= (b & 127) << s; s += 7; } while (b & 128); return [v, at]; };
    let n; [n, q] = rb(q);
    const ents = [];
    for (let i = 0; i < n; i++) {
      let len; [len, q] = rb(q);
      const name = Buffer.from(body.subarray(q, q + len)).toString(); q += len;
      const kind = body[q++];
      let idx; [idx, q] = rb(q);
      if (name.startsWith("__")) continue;
      const nn = Buffer.from(rename[name] ?? name);
      ents.push([...wr(nn.length), ...nn, kind, ...wr(idx)]);
    }
    const sec = [...wr(ents.length), ...ents.flat()];
    out.push(7, ...wr(sec.length), ...sec);
  }
  return Buffer.from(out);
}
function xor(buf) {
  const out = Buffer.alloc(buf.length);
  let s = 0x9e3779b1 | 0;
  for (let i = 0; i < buf.length; i++) {
    s = (Math.imul(s, 1664525) + 1013904223) | 0;
    out[i] = buf[i] ^ (s >>> 24);
  }
  return out;
}

const wasm = scrub(readFileSync(opt));
const WATCH = [
  "document", "body", "canvas", "appendChild", "getContext", "alpha", "style",
  "cssText", "width", "height", "ResizeObserver", "observe", "Object",
  "devicePixelContentBoxSize", "inlineSize", "blockSize", "devicePixelRatio",
  "innerWidth", "innerHeight", "requestAnimationFrame", "performance",
  "oncontextmenu", "preventDefault", "head", "link", "rel", "icon", "href",
  "toDataURL", "textBaseline", "middle", "textAlign", "center", "fillStyle",
  "globalAlpha", "font", "fillRect", "fillText", "beginPath", "moveTo",
  "lineTo", "bezierCurveTo", "fill", "arc", "resize", "frame", "memory",
  "__heap", "__data",
];
const rawText = wasm.toString("latin1");
const leaks = WATCH.filter((w) => rawText.includes(w));
if (leaks.length) {
  console.error("plaintext leak in wasm, refusing to build:", leaks.join(", "));
  process.exit(1);
}

const mod = new WebAssembly.Module(wasm);
const imports = WebAssembly.Module.imports(mod).map((i) => `${i.module}.${i.name}`).join(" ");
const exports = WebAssembly.Module.exports(mod).map((e) => e.name).join(" ");

const template = readFileSync(`${root}/index.html`, "utf8");
if (!/atob\("[^"]*"\)/.test(template)) throw new Error("atob slot not found in index.html");
const page = template.replace(/atob\("[^"]*"\)/, () => `atob("${xor(wasm).toString("base64")}")`);

mkdirSync(dirname(outPath), { recursive: true });
writeFileSync(outPath, page);

console.log(`raw ${readFileSync(raw).length}  opt ${wasm.length}  html ${page.length} bytes`);
console.log("imports", imports);
console.log("exports", exports);
console.log("wrote", outPath);
