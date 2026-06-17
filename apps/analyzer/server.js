import { createServer } from "node:http";
import { execFile } from "node:child_process";
import { mkdtemp, rm, readdir, readFile } from "node:fs/promises";
import { existsSync } from "node:fs";
import { tmpdir } from "node:os";
import { createRequire } from "node:module";
import path from "node:path";
import { fileURLToPath } from "node:url";

import AdmZip from "adm-zip";

import { normalize } from "./normalize.js";

const require = createRequire(import.meta.url);
const __dirname = path.dirname(fileURLToPath(import.meta.url));

const PORT = Number(process.env.PORT || 3000);
const MAX_ARCHIVE_BYTES = Number(process.env.MAX_ARCHIVE_BYTES || 64 * 1024 * 1024);
const MAX_UNCOMPRESSED_BYTES = Number(process.env.MAX_UNCOMPRESSED_BYTES || 512 * 1024 * 1024);
const MAX_ENTRIES = Number(process.env.MAX_ENTRIES || 20000);
const MAX_CONCURRENCY = Number(process.env.MAX_CONCURRENCY || 2);
const VALIDATE_TIMEOUT_MS = Number(process.env.VALIDATE_TIMEOUT_MS || 60000);
const DEFAULT_SUITE = process.env.DEFAULT_SUITE || "addon";
const ALLOWED_SUITES = new Set(["all", "default", "addon", "currentplatform", "main"]);

const isWin = process.platform === "win32";
const MCT_BIN = process.env.MCT_BIN || path.join(__dirname, "node_modules", ".bin", isWin ? "mct.cmd" : "mct");

function mctoolsVersion() {
	try {
		return require("@minecraft/creator-tools/package.json").version || "";
	} catch {
		return "";
	}
}

let inFlight = 0;

function sendJson(res, status, body) {
	const payload = JSON.stringify(body);
	res.writeHead(status, {
		"content-type": "application/json",
		"content-length": Buffer.byteLength(payload),
	});
	res.end(payload);
}

function readBody(req, limit) {
	return new Promise((resolve, reject) => {
		const chunks = [];
		let total = 0;
		req.on("data", (chunk) => {
			total += chunk.length;
			if (total > limit) {
				reject(Object.assign(new Error("archive too large"), { code: 413 }));
				req.destroy();
				return;
			}
			chunks.push(chunk);
		});
		req.on("end", () => resolve(Buffer.concat(chunks)));
		req.on("error", reject);
	});
}

function extract(buffer, dest) {
	const zip = new AdmZip(buffer);
	const entries = zip.getEntries();
	if (entries.length > MAX_ENTRIES) {
		throw Object.assign(new Error("too many entries"), { code: 422 });
	}
	let uncompressed = 0;
	for (const entry of entries) {
		uncompressed += entry.header.size;
		if (uncompressed > MAX_UNCOMPRESSED_BYTES) {
			throw Object.assign(new Error("archive expands too large"), {
				code: 422,
			});
		}
		const target = path.resolve(dest, entry.entryName);
		if (target !== dest && !target.startsWith(dest + path.sep)) {
			throw Object.assign(new Error("unsafe path in archive"), { code: 422 });
		}
	}
	zip.extractAllTo(dest, true);
}

function runValidate(suite, inputDir, outputDir) {
	return new Promise((resolve, reject) => {
		execFile(
			MCT_BIN,
			["validate", suite, "-i", inputDir, "-o", outputDir, "--json"],
			{
				timeout: VALIDATE_TIMEOUT_MS,
				killSignal: "SIGKILL",
				maxBuffer: 64 * 1024 * 1024,
				shell: isWin,
				windowsHide: true,
			},
			(error, stdout) => {
				if (error && error.killed) {
					reject(Object.assign(new Error("validation timed out"), { code: 504 }));
					return;
				}
				resolve(stdout || "");
			}
		);
	});
}

function parseCliJson(stdout) {
	const lines = stdout
		.split(/\r?\n/)
		.map((l) => l.trim())
		.filter((l) => l.startsWith("{") && l.endsWith("}"));
	for (let i = lines.length - 1; i >= 0; i -= 1) {
		try {
			return JSON.parse(lines[i]);
		} catch {
			// keep scanning
		}
	}
	return null;
}

async function readMcrReport(outputDir) {
	let files;
	try {
		files = await readdir(outputDir);
	} catch {
		return null;
	}
	const reportName = files.find((f) => f.endsWith(".mcr.json"));
	if (!reportName) return null;
	try {
		const raw = await readFile(path.join(outputDir, reportName), "utf8");
		return JSON.parse(raw);
	} catch {
		return null;
	}
}

async function analyze(buffer, suite) {
	const workDir = await mkdtemp(path.join(tmpdir(), "mct-"));
	const inputDir = path.join(workDir, "pack");
	const outputDir = path.join(workDir, "out");
	try {
		extract(buffer, inputDir);
		const stdout = await runValidate(suite, inputDir, outputDir);
		const cli = parseCliJson(stdout);
		const mcr = await readMcrReport(outputDir);
		if (!cli && !mcr) {
			throw Object.assign(new Error("no report produced"), { code: 502 });
		}
		return normalize({ cli, mcr, mctoolsVersion: mctoolsVersion() });
	} finally {
		await rm(workDir, { recursive: true, force: true }).catch(() => {});
	}
}

const server = createServer(async (req, res) => {
	const url = new URL(req.url, "http://localhost");

	if (req.method === "GET" && url.pathname === "/health") {
		if (!existsSync(MCT_BIN)) {
			sendJson(res, 503, { status: "degraded", reason: "mct binary missing" });
			return;
		}
		sendJson(res, 200, { status: "ok", mctoolsVersion: mctoolsVersion() });
		return;
	}

	if (req.method === "POST" && url.pathname === "/analyze") {
		if (inFlight >= MAX_CONCURRENCY) {
			sendJson(res, 429, { error: "analyzer busy, retry later" });
			return;
		}
		const suite = url.searchParams.get("suite") || DEFAULT_SUITE;
		if (!ALLOWED_SUITES.has(suite)) {
			sendJson(res, 400, { error: "invalid suite" });
			return;
		}

		inFlight += 1;
		try {
			const buffer = await readBody(req, MAX_ARCHIVE_BYTES);
			if (buffer.length === 0) {
				sendJson(res, 400, { error: "empty body" });
				return;
			}
			const report = await analyze(buffer, suite);
			sendJson(res, 200, report);
		} catch (err) {
			const code = typeof err?.code === "number" ? err.code : 500;
			sendJson(res, code, { error: err?.message || "analysis failed" });
		} finally {
			inFlight -= 1;
		}
		return;
	}

	sendJson(res, 404, { error: "not found" });
});

server.listen(PORT, () => {
	console.log(`mctools-analyzer listening on :${PORT} (mct ${mctoolsVersion()})`);
});
