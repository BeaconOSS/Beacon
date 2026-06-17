export type DiffLineType = "context" | "add" | "remove";

export interface DiffLine {
  type: DiffLineType;
  oldNumber: number | null;
  newNumber: number | null;
  text: string;
}

export interface LineDiff {
  lines: DiffLine[];
  added: number;
  removed: number;
  truncated: boolean;
  tooLarge: boolean;
}

const MAX_DIFF_LINES = 4000;
const MAX_RENDER_LINES = 1500;

export function diffLines(oldText: string, newText: string): LineDiff {
  const a = oldText.length ? oldText.split("\n") : [];
  const b = newText.length ? newText.split("\n") : [];

  if (a.length > MAX_DIFF_LINES || b.length > MAX_DIFF_LINES) {
    return {
      lines: [],
      added: 0,
      removed: 0,
      truncated: false,
      tooLarge: true,
    };
  }

  const n = a.length;
  const m = b.length;

  const lcs: number[][] = Array.from({ length: n + 1 }, () =>
    new Array<number>(m + 1).fill(0),
  );
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      lcs[i]![j] =
        a[i] === b[j]
          ? lcs[i + 1]![j + 1]! + 1
          : Math.max(lcs[i + 1]![j]!, lcs[i]![j + 1]!);
    }
  }

  const lines: DiffLine[] = [];
  let added = 0;
  let removed = 0;
  let i = 0;
  let j = 0;

  while (i < n && j < m) {
    if (a[i] === b[j]) {
      lines.push({
        type: "context",
        oldNumber: i + 1,
        newNumber: j + 1,
        text: a[i]!,
      });
      i++;
      j++;
    } else if (lcs[i + 1]![j]! >= lcs[i]![j + 1]!) {
      lines.push({
        type: "remove",
        oldNumber: i + 1,
        newNumber: null,
        text: a[i]!,
      });
      removed++;
      i++;
    } else {
      lines.push({
        type: "add",
        oldNumber: null,
        newNumber: j + 1,
        text: b[j]!,
      });
      added++;
      j++;
    }
  }
  while (i < n) {
    lines.push({
      type: "remove",
      oldNumber: i + 1,
      newNumber: null,
      text: a[i]!,
    });
    removed++;
    i++;
  }
  while (j < m) {
    lines.push({ type: "add", oldNumber: null, newNumber: j + 1, text: b[j]! });
    added++;
    j++;
  }

  const truncated = lines.length > MAX_RENDER_LINES;
  return {
    lines: truncated ? lines.slice(0, MAX_RENDER_LINES) : lines,
    added,
    removed,
    truncated,
    tooLarge: false,
  };
}

export interface WordDiffSegment {
  type: DiffLineType;
  text: string;
}

export interface WordDiff {
  segments: WordDiffSegment[];
  changed: boolean;
  tooLarge: boolean;
}

const MAX_DIFF_TOKENS = 2000;

function tokenize(text: string): string[] {
  return text.match(/\s+|\S+/g) ?? [];
}

export function diffWords(oldText: string, newText: string): WordDiff {
  const changed = oldText !== newText;
  const a = tokenize(oldText);
  const b = tokenize(newText);

  if (a.length > MAX_DIFF_TOKENS || b.length > MAX_DIFF_TOKENS) {
    return { segments: [], changed, tooLarge: true };
  }

  const n = a.length;
  const m = b.length;
  const lcs: number[][] = Array.from({ length: n + 1 }, () =>
    new Array<number>(m + 1).fill(0),
  );
  for (let i = n - 1; i >= 0; i--) {
    for (let j = m - 1; j >= 0; j--) {
      lcs[i]![j] =
        a[i] === b[j]
          ? lcs[i + 1]![j + 1]! + 1
          : Math.max(lcs[i + 1]![j]!, lcs[i]![j + 1]!);
    }
  }

  const raw: WordDiffSegment[] = [];
  let i = 0;
  let j = 0;
  while (i < n && j < m) {
    if (a[i] === b[j]) {
      raw.push({ type: "context", text: a[i]! });
      i++;
      j++;
    } else if (lcs[i + 1]![j]! >= lcs[i]![j + 1]!) {
      raw.push({ type: "remove", text: a[i]! });
      i++;
    } else {
      raw.push({ type: "add", text: b[j]! });
      j++;
    }
  }
  while (i < n) {
    raw.push({ type: "remove", text: a[i]! });
    i++;
  }
  while (j < m) {
    raw.push({ type: "add", text: b[j]! });
    j++;
  }

  const segments: WordDiffSegment[] = [];
  for (const seg of raw) {
    const last = segments[segments.length - 1];
    if (last && last.type === seg.type) {
      last.text += seg.text;
    } else {
      segments.push({ ...seg });
    }
  }

  return { segments, changed, tooLarge: false };
}

export function looksLikePlaceholder(text: string): boolean {
  const trimmed = text.trim();
  if (trimmed.length < 12) return false;
  const lower = trimmed.toLowerCase();

  const compact = lower.replace(/\s+/g, "");
  for (let len = 1; len <= 8 && len < compact.length; len++) {
    const unit = compact.slice(0, len);
    if (
      unit.repeat(Math.ceil(compact.length / len)).slice(0, compact.length) ===
      compact
    ) {
      return true;
    }
  }

  const words = lower.split(/\s+/).filter(Boolean);
  if (words.length >= 5 && new Set(words).size / words.length < 0.3) {
    return true;
  }

  return false;
}

const TEXT_EXTENSIONS = new Set([
  "json",
  "geo",
  "material",
  "lang",
  "mcfunction",
  "txt",
  "properties",
  "js",
  "ts",
  "html",
  "css",
  "md",
]);

const IMAGE_EXTENSIONS = new Set(["png", "jpg", "jpeg", "gif", "webp"]);

function extension(path: string): string {
  const name = path.split("/").pop() ?? path;
  const dot = name.lastIndexOf(".");
  return dot >= 0 ? name.slice(dot + 1).toLowerCase() : "";
}

export type FilePreviewKind = "text" | "image" | "binary";

export function previewKind(path: string): FilePreviewKind {
  const ext = extension(path);
  if (TEXT_EXTENSIONS.has(ext)) return "text";
  if (IMAGE_EXTENSIONS.has(ext)) return "image";
  return "binary";
}
