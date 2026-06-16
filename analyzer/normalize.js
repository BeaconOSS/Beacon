const FINDING_TYPES = new Set([
  "error",
  "testFail",
  "warn",
  "warning",
  "testWarn",
  "recommendation",
]);

function num(value, fallback = 0) {
  return typeof value === "number" && Number.isFinite(value) ? value : fallback;
}

function str(value, fallback = "") {
  return typeof value === "string" ? value : fallback;
}

function arr(value) {
  return Array.isArray(value) ? value : [];
}

function pickInfo(info) {
  return {
    capabilities: arr(info.capabilities),
    apisUsed: arr(info.apisUsed),
    behaviorPackManifestCount: num(info.behaviorPackManifestCount),
    resourcePackManifestCount: num(info.resourcePackManifestCount),
    entityTypeManifestCount: num(info.entityTypeManifestCount),
    itemTypeManifestCount: num(info.itemTypeManifestCount),
    blockTypeManifestCount: num(info.blockTypeManifestCount),
    worldCount: num(info.worldCount),
    subpackCount: num(info.subpackCount),
    overallSize: num(info.overallSize),
    contentSize: num(info.contentSize),
    fileCounts: num(info.fileCounts),
    folderCounts: num(info.folderCounts),
    contentFileCounts: num(info.contentFileCounts),
    animationCount: num(info.animationCount),
    textureCount: num(info.textureCount),
    vanillaGameTextureCoverage: num(info.vanillaGameTextureCoverage),
    minBehaviorPackMinEngineVersion: num(info.minBehaviorPackMinEngineVersion),
    minResourcePackMinEngineVersion: num(info.minResourcePackMinEngineVersion),
    itemTypes: arr(info.itemTypes),
  };
}

export function normalize({ cli, mcr, mctoolsVersion }) {
  const info = (mcr && typeof mcr.info === "object" && mcr.info) || {};
  const project = (cli && Array.isArray(cli.projects) && cli.projects[0]) || {};
  const items = arr(project.items);

  const findings = items
    .filter((it) => it && FINDING_TYPES.has(it.type))
    .map((it) => ({
      type: str(it.type),
      generatorId: str(it.generatorId),
      message: str(it.message),
    }));

  const features = items
    .filter(
      (it) =>
        it &&
        it.type === "featureAggregate" &&
        typeof it.message === "string" &&
        it.message.length > 0,
    )
    .map((it) => ({
      generatorId: str(it.generatorId),
      label: str(it.message),
      data: num(it.data),
    }));

  const counts = {
    errors: num(cli && cli.errors, num(info.errorCount)),
    warnings: num(cli && cli.warnings, num(info.warningCount)),
    recommendations: num(cli && cli.recommendations),
    testSuccess: num(info.testSuccessCount),
    testFail: num(info.testFailCount),
    testNotApplicable: num(info.testNotApplicableCount),
  };

  const decision =
    counts.errors > 0 ? "fail" : counts.warnings > 0 ? "warn" : "pass";

  return {
    schemaVersion: str(cli && cli.schemaVersion),
    mctoolsVersion: str(mctoolsVersion),
    mctoolsVersionRaw: num(info.mctoolsVersion),
    decision,
    counts,
    info: pickInfo(info),
    findings,
    features,
    summaries: {
      error: str(info.errorSummary),
      warning: str(info.warningSummary),
      testFail: str(info.testFailSummary),
    },
  };
}
