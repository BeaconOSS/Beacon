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
