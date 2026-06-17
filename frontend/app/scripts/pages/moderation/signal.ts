import { looksLikePlaceholder } from "~/scripts/pages/diff";
import type { PendingReview } from "~/scripts/pages/projects/types";
import { SIGNAL_RANK, plural } from "./meta";
import type { DecisionCheck, DecisionSignal, SignalStatus } from "./types";

export function buildDecisionSignal(
  review: PendingReview | null,
): DecisionSignal {
  const checks: DecisionCheck[] = [];

  const analysis = review?.analysis ?? null;
  let validationStatus: SignalStatus = "pending";
  if (!analysis || analysis.status === "pending") {
    checks.push({
      label: "MCTools validation",
      status: "pending",
      detail: analysis ? "Analysis in progress" : "Not yet run",
    });
  } else if (analysis.status === "error") {
    validationStatus = "warn";
    checks.push({
      label: "MCTools validation",
      status: "warn",
      detail: "Could not validate - review manually",
    });
  } else if (analysis.report) {
    const decision = analysis.report.decision;
    validationStatus =
      decision === "fail" ? "fail" : decision === "warn" ? "warn" : "pass";
    const counts = analysis.report.counts;
    checks.push({
      label: "MCTools validation",
      status: validationStatus,
      detail:
        decision === "pass"
          ? "No errors or warnings"
          : `${plural(counts.errors, "error")}, ${plural(counts.warnings, "warning")}`,
    });
  } else {
    checks.push({
      label: "MCTools validation",
      status: "pending",
      detail: "No report available",
    });
  }

  if (analysis?.status === "ready" && analysis.report) {
    const counts = analysis.report.counts;
    if (counts.testFail > 0) {
      checks.push({
        label: "Automated tests",
        status: "fail",
        detail: `${counts.testFail} failed, ${counts.testSuccess} passed`,
      });
    } else if (counts.testSuccess > 0) {
      checks.push({
        label: "Automated tests",
        status: "pass",
        detail: plural(counts.testSuccess, "test") + " passed",
      });
    } else {
      checks.push({
        label: "Automated tests",
        status: "neutral",
        detail: "No tests applicable",
      });
    }
  }

  const diff = review?.pack_diff ?? null;
  if (diff?.files_truncated) {
    checks.push({
      label: "Pack contents",
      status: "warn",
      detail: "Too many changes to index fully",
    });
  } else if (diff && diff.added + diff.removed + diff.modified > 0) {
    checks.push({
      label: "Pack contents",
      status: "pass",
      detail:
        plural(diff.added + diff.removed + diff.modified, "file change") +
        " indexed",
    });
  }

  const note = review?.changelog?.trim() ?? "";
  checks.push(
    note
      ? { label: "Creator note", status: "pass", detail: "Provided" }
      : {
          label: "Creator note",
          status: "warn",
          detail: "No changelog provided",
        },
  );

  if (review) {
    const content = review.pending;
    const title = content.title.trim();
    const summary = content.summary.trim();
    const description = content.description.trim();
    const smells: DecisionCheck[] = [];

    if (!description) {
      smells.push({ label: "Description", status: "warn", detail: "Empty" });
    } else if (looksLikePlaceholder(description)) {
      smells.push({
        label: "Description",
        status: "warn",
        detail: "Looks like placeholder text",
      });
    } else if (description.length < 30) {
      smells.push({
        label: "Description",
        status: "warn",
        detail: "Very short",
      });
    }

    if (summary && looksLikePlaceholder(summary)) {
      smells.push({
        label: "Summary",
        status: "warn",
        detail: "Looks like placeholder text",
      });
    } else if (
      summary &&
      title &&
      summary.toLowerCase() === title.toLowerCase()
    ) {
      smells.push({
        label: "Summary",
        status: "warn",
        detail: "Duplicates the title",
      });
    }

    const insecureLinks = Object.values(review.links).filter((url) =>
      url.trim().toLowerCase().startsWith("http://"),
    );
    if (insecureLinks.length > 0) {
      smells.push({
        label: "Links",
        status: "warn",
        detail: plural(insecureLinks.length, "insecure (http) link"),
      });
    }

    if (review.gallery.length === 0) {
      smells.push({ label: "Gallery", status: "neutral", detail: "No images" });
    }

    if (/all rights reserved/i.test(content.license)) {
      smells.push({
        label: "License",
        status: "neutral",
        detail: "All Rights Reserved",
      });
    }

    if (smells.length === 0) {
      checks.push({
        label: "Listing content",
        status: "pass",
        detail: "No issues detected",
      });
    } else {
      checks.push(...smells);
    }
  }

  const overall: SignalStatus =
    validationStatus === "pending"
      ? "pending"
      : checks.reduce<SignalStatus>(
          (worst, check) =>
            SIGNAL_RANK[check.status] > SIGNAL_RANK[worst]
              ? check.status
              : worst,
          "pass",
        );

  return { overall, checks };
}
