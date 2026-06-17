import { execFileSync } from "node:child_process";
import { readFileSync, writeFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = join(__dirname, "..");
const FRONTEND = join(ROOT, "apps", "frontend");
const BACKEND = join(ROOT, "apps", "backend");
const OUT = join(FRONTEND, "app", "data", "licenses.json");
const LICENSE_URLS = {
  MIT: "https://opensource.org/license/mit",
  "Apache-2.0": "https://www.apache.org/licenses/LICENSE-2.0",
  ISC: "https://opensource.org/license/isc-license-txt",
  "BSD-2-Clause": "https://opensource.org/license/bsd-2-clause",
  "BSD-3-Clause": "https://opensource.org/license/bsd-3-clause",
  "0BSD": "https://opensource.org/license/0bsd",
  "MPL-2.0": "https://www.mozilla.org/en-US/MPL/2.0/",
  Unlicense: "https://unlicense.org/",
  "CC0-1.0": "https://creativecommons.org/publicdomain/zero/1.0/",
  Zlib: "https://opensource.org/license/zlib",
};

// Nicer display for some packages
const DISPLAY = {
  // frontend
  nuxt: "Nuxt",
  vue: "Vue",
  "vue-router": "Vue Router",
  "reka-ui": "Reka UI",
  "@vueuse/core": "VueUse",
  "@lucide/vue": "Lucide",
  "@nuxt/eslint": "Nuxt ESLint",
  "@tailwindcss/vite": "@tailwindcss/vite",
  tailwindcss: "Tailwind CSS",
  eslint: "ESLint",
  typescript: "TypeScript",
  "vue-tsc": "Vue Language Tools",
  // backend
  argon2: "Argon2",
  axum: "Axum",
  serde: "Serde",
  sha2: "SHA-2",
  sqlx: "SQLx",
  tokio: "Tokio",
};

function resolveLicense(spdx) {
  if (!spdx) return { license: "Unknown", licenseUrl: "" };
  const tokens = spdx
    .replace(/[()]/g, " ")
    .split(/\s+(?:OR|AND|\/)\s+/i)
    .map((t) => t.trim())
    .filter(Boolean);
  const display = tokens.join(" / ");
  const first = tokens.find((t) => LICENSE_URLS[t]) ?? tokens[0];
  return { license: display, licenseUrl: LICENSE_URLS[first] ?? "" };
}

function normalizeRepo(repository) {
  if (!repository) return "";
  let url = typeof repository === "string" ? repository : repository.url || "";
  if (!url) return "";
  const shorthand = url.match(/^github:(.+)$/i);
  if (shorthand) return `https://github.com/${shorthand[1]}`;
  if (/^[\w.-]+\/[\w.-]+$/.test(url)) return `https://github.com/${url}`;
  url = url
    .replace(/^git\+/, "")
    .replace(/^git:\/\//, "https://")
    .replace(/^git@([^:]+):/, "https://$1/")
    .replace(/\.git$/, "");
  return url;
}

function display(name) {
  return DISPLAY[name] ?? name;
}

function toLibrary(name, spdx, repository) {
  const { license, licenseUrl } = resolveLicense(spdx);
  return {
    name: display(name),
    url: normalizeRepo(repository),
    license,
    licenseUrl,
  };
}

function byName(a, b) {
  return a.name.localeCompare(b.name, "en", { sensitivity: "base" });
}

function collectFrontend() {
  const pkg = JSON.parse(readFileSync(join(FRONTEND, "package.json"), "utf8"));
  const names = [
    ...Object.keys(pkg.dependencies ?? {}),
    ...Object.keys(pkg.devDependencies ?? {}),
  ];
  const libraries = [];
  for (const name of names) {
    const manifest = join(FRONTEND, "node_modules", name, "package.json");
    if (!existsSync(manifest)) {
      console.warn(`! skipping ${name} (not found in node_modules)`);
      continue;
    }
    const meta = JSON.parse(readFileSync(manifest, "utf8"));
    const spdx =
      typeof meta.license === "string"
        ? meta.license
        : (meta.license?.type ??
          (Array.isArray(meta.licenses)
            ? meta.licenses.map((l) => l.type).join(" OR ")
            : ""));
    libraries.push(toLibrary(name, spdx, meta.repository));
  }
  return libraries.sort(byName);
}

function collectBackend() {
  const raw = execFileSync(
    "cargo",
    [
      "metadata",
      "--format-version",
      "1",
      "--manifest-path",
      join(BACKEND, "Cargo.toml"),
    ],
    { encoding: "utf8", maxBuffer: 64 * 1024 * 1024 },
  );
  const meta = JSON.parse(raw);
  const root = meta.packages.find((p) => p.name === "beacon-backend");
  if (!root) throw new Error("could not find beacon-backend in cargo metadata");
  const directNames = new Set(
    root.dependencies.filter((d) => d.kind == null).map((d) => d.name),
  );
  const libraries = [];
  for (const name of directNames) {
    const pkg = meta.packages.find((p) => p.name === name);
    if (!pkg) {
      console.warn(`! skipping crate ${name} (not in metadata)`);
      continue;
    }
    libraries.push(toLibrary(name, pkg.license, pkg.repository));
  }
  return libraries.sort(byName);
}

const data = {
  generatedAt: new Date().toISOString(),
  groups: [
    {
      title: "Frontend",
      description: "The website you're looking at is built with these.",
      libraries: collectFrontend(),
    },
    {
      title: "Backend",
      description: "Our API and services run on these.",
      libraries: collectBackend(),
    },
  ],
};

mkdirSync(dirname(OUT), { recursive: true });
writeFileSync(OUT, JSON.stringify(data, null, 2) + "\n");
const total = data.groups.reduce((n, g) => n + g.libraries.length, 0);
console.log(`Wrote ${total} libraries to ${OUT}`);
