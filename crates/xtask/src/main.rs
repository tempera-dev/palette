//! Workspace automation tasks, invoked via `cargo xtask <command>`.
//!
//! The project standard is `cargo xtask` over ad-hoc shell scripts. This crate
//! owns the OpenAPI spec / SDK regeneration pipeline:
//!
//! * `regen-spec`: regenerate `sdks/openapi/beater-api.json`, the dashboard
//!   snapshot, AND the dashboard's typed client
//!   (`web/dashboard/lib/generated/api-types.ts`) directly from the `beater-api`
//!   handlers — every spec-derived artifact in one step, so nothing the CI drift
//!   checks enforce is left stale. The dashboard client step needs Node/npx.
//! * `regen-sdks`: regenerate the spec *and* every language client by shelling
//!   out to `scripts/regen-sdks.sh` (needs Docker).
//! * `check-drift`: regenerate the spec into a temp file and `git diff` the
//!   committed spec/dashboard snapshot, failing if stale. This is the fast,
//!   Docker-free drift check; the full check is `cargo xtask regen-sdks`
//!   followed by `git diff`.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{bail, Context};
use clap::{Parser, Subcommand};
use regex::Regex;

/// Canonical spec location, relative to the workspace root.
const SPEC_PATH: &str = "sdks/openapi/beater-api.json";
/// Cross-language semantic-conventions contract, regenerated from beater-schema.
const SEMCONV_PATH: &str = "sdks/semconv/conventions.json";
/// Dashboard snapshot that must stay byte-identical to the canonical spec.
const DASHBOARD_SPEC_PATH: &str = "web/dashboard/openapi/beater-read-api.json";
/// Hand-maintained architecture status ledger that `check-arch-status` guards.
const ARCH_STATUS_PATH: &str = "docs/architecture-status.md";
/// Workspace manifest whose `members` list is the source of truth for which
/// crates/bins actually exist in the build graph.
const WORKSPACE_MANIFEST_PATH: &str = "Cargo.toml";

#[derive(Debug, Parser)]
#[command(name = "xtask", about = "Beater workspace automation tasks")]
struct Args {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Debug, Subcommand)]
enum Cmd {
    /// Regenerate the OpenAPI spec from the beater-api handlers (no Docker).
    RegenSpec,
    /// Regenerate the spec and every language SDK client (requires Docker).
    RegenSdks,
    /// Fail if the committed spec is stale versus the handlers (no Docker).
    CheckDrift,
    /// Regenerate the cross-language semconv contract from beater-schema.
    RegenSemconv,
    /// Guard `docs/architecture-status.md` against drift from the repo (no Docker).
    CheckArchStatus,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Cmd::RegenSpec => regen_spec(),
        Cmd::RegenSdks => regen_sdks(),
        Cmd::CheckDrift => check_drift(),
        Cmd::RegenSemconv => regen_semconv(),
        Cmd::CheckArchStatus => check_arch_status(),
    }
}

/// Write `sdks/semconv/conventions.json` from `beater_schema::conventions`, the
/// single source of truth for span kinds + attribute keys. CI regenerates this
/// and fails on `git diff`, so the server contract can't drift from the file the
/// SDK drift checker validates against.
fn regen_semconv() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let path = root.join(SEMCONV_PATH);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create semconv dir {}", parent.display()))?;
    }
    std::fs::write(&path, beater_schema::conventions::conventions_json())
        .with_context(|| format!("write semconv {}", path.display()))?;
    println!("wrote {}", path.display());
    Ok(())
}

/// Locate the workspace root from this crate's manifest directory.
///
/// `crates/xtask` is two levels below the workspace root.
fn workspace_root() -> anyhow::Result<PathBuf> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .ancestors()
        .nth(2)
        .map(Path::to_path_buf)
        .context("locate workspace root from xtask manifest dir")
}

/// Render the canonical OpenAPI document directly from the handlers.
fn render_spec() -> anyhow::Result<String> {
    let mut json =
        beater_api::openapi::openapi_json_pretty().context("render OpenAPI spec to JSON")?;
    // Match the trailing newline that shell redirection (`> file`) produced for
    // the committed artifact so drift checks compare equal.
    json.push('\n');
    Ok(json)
}

fn regen_spec() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let spec = render_spec()?;
    let spec_path = root.join(SPEC_PATH);
    let dashboard_path = root.join(DASHBOARD_SPEC_PATH);

    if let Some(parent) = spec_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create spec dir {}", parent.display()))?;
    }
    if let Some(parent) = dashboard_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("create dashboard spec dir {}", parent.display()))?;
    }

    std::fs::write(&spec_path, &spec)
        .with_context(|| format!("write spec {}", spec_path.display()))?;
    std::fs::write(&dashboard_path, &spec)
        .with_context(|| format!("write dashboard spec {}", dashboard_path.display()))?;

    println!("wrote {}", spec_path.display());
    println!("wrote {}", dashboard_path.display());

    // The dashboard's typed client is a THIRD spec-derived artifact. Regenerate
    // it here so `regen-spec` leaves nothing stale — otherwise CI's
    // `check-openapi-drift.sh` (the `verify` job) flags drift even after the
    // documented regen steps were run.
    regen_dashboard_client(&root)?;
    Ok(())
}

/// Regenerate `web/dashboard/lib/generated/api-types.ts` from the dashboard spec
/// snapshot using the exact `openapi-typescript` version pinned in the
/// dashboard's `package.json` (so the output matches CI byte-for-byte). `npx
/// --yes` fetches that version on demand, so a prior `npm ci` is not required.
fn regen_dashboard_client(root: &Path) -> anyhow::Result<()> {
    let dashboard = root.join("web/dashboard");
    let version = openapi_typescript_version(&dashboard)?;
    let out_rel = "lib/generated/api-types.ts";
    let status = Command::new("npx")
        .args([
            "--yes",
            &format!("openapi-typescript@{version}"),
            "openapi/beater-read-api.json",
            "-o",
            out_rel,
        ])
        .current_dir(&dashboard)
        .status()
        .with_context(|| {
            "run `npx openapi-typescript` in web/dashboard (is Node/npm installed?)"
        })?;
    if !status.success() {
        bail!("dashboard api-types.ts generation failed (openapi-typescript@{version})");
    }
    println!("wrote {}", dashboard.join(out_rel).display());
    Ok(())
}

/// Read the pinned `openapi-typescript` version from the dashboard's
/// `package.json` devDependencies without taking a JSON parser dependency.
fn openapi_typescript_version(dashboard: &Path) -> anyhow::Result<String> {
    let pkg = std::fs::read_to_string(dashboard.join("package.json"))
        .with_context(|| "read web/dashboard/package.json")?;
    for line in pkg.lines() {
        if let Some(rest) = line.trim().strip_prefix("\"openapi-typescript\":") {
            let version: String = rest
                .trim()
                .trim_start_matches('"')
                .chars()
                .take_while(|ch| *ch != '"')
                .collect();
            let version = version.trim_start_matches(['^', '~']).to_string();
            if !version.is_empty() {
                return Ok(version);
            }
        }
    }
    bail!("openapi-typescript is not pinned in web/dashboard/package.json devDependencies")
}

fn regen_sdks() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let script = root.join("scripts/regen-sdks.sh");
    let status = Command::new("bash")
        .arg(&script)
        .current_dir(&root)
        .status()
        .with_context(|| format!("run {}", script.display()))?;
    if !status.success() {
        bail!(
            "{} failed with {}",
            script.display(),
            status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "signal".to_string())
        );
    }
    Ok(())
}

fn check_drift() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let fresh = render_spec()?;

    // Render the canonical spec into a temp file, then `git diff --no-index`
    // each committed artifact against it. `--no-index` gives us a real diff
    // (and a clean non-zero exit) regardless of whether the on-disk file is
    // tracked, staged, or untracked -- we only care that the *content* the
    // handlers produce matches what is checked in.
    let dir = std::env::temp_dir().join(format!("beater-xtask-drift-{}", std::process::id()));
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("create temp drift dir {}", dir.display()))?;
    let result = check_drift_inner(&root, &fresh, &dir);
    // Best-effort cleanup; ignore failures so they never mask the real result.
    let _ = std::fs::remove_dir_all(&dir);
    result
}

fn check_drift_inner(root: &Path, fresh: &str, dir: &Path) -> anyhow::Result<()> {
    let temp_spec = dir.join("beater-api.json");
    std::fs::write(&temp_spec, fresh)
        .with_context(|| format!("write temp spec {}", temp_spec.display()))?;

    for relative in [SPEC_PATH, DASHBOARD_SPEC_PATH] {
        let committed = root.join(relative);
        // `git diff --no-index --exit-code` returns 1 when the files differ.
        let status = Command::new("git")
            .args(["diff", "--no-index", "--exit-code", "--"])
            .arg(&committed)
            .arg(&temp_spec)
            .current_dir(root)
            .status()
            .with_context(|| format!("run git diff for {relative}"))?;
        if !status.success() {
            bail!(
                "OpenAPI spec drift: {relative} is stale versus the beater-api handlers.\n\
                 Run `cargo xtask regen-spec` (and `cargo xtask regen-sdks` for the language \
                 clients) and commit. The full check is `cargo xtask regen-sdks` followed by \
                 `git diff` over sdks/clients."
            );
        }
    }

    println!("no drift: OpenAPI spec is current");
    Ok(())
}

// ---------------------------------------------------------------------------
// check-arch-status
// ---------------------------------------------------------------------------
//
// `docs/architecture-status.md` is a hand-maintained ledger of which surfaces
// are built / planned / deferred. It drifts from reality (issue #296). This is
// a pragmatic, deterministic drift guard — NOT a semantic auditor. It parses
// the markdown table rows line-by-line and enforces three classes of invariant:
//
//   1. Every `beater-*` (or `xtask`) crate name mentioned in the ledger must be
//      a real workspace member (parsed from `Cargo.toml [workspace].members`).
//      Catches references to crates that were renamed or never existed.
//   2. A small curated list of "must-exist, must-be-marked-built" surfaces is
//      checked against the tree (file/symbol presence). This catches the
//      specific failure in #296: a surface that exists in code but is still
//      written up as planned. MCP stdio (`beaterd mcp --stdio` +
//      `beater_mcp::serve_stdio`) is the flagship entry here.
//   3. Every row whose status cell says `planned`/`deferred` must either carry a
//      GitHub issue/PR link (`#123` or a github.com URL) OR appear in an
//      explicit allowlist below (rows that are honestly planned but have no
//      tracking issue yet). The allowlist keeps the gate green while making the
//      "no untracked planned row" intent auditable in one place.
//
// Keep this simple. A maintainable, deterministic check beats a clever one.

/// A curated surface the ledger MUST describe as built, with a cheap proof that
/// it really exists in the tree. `needle` is searched verbatim inside `file`.
struct MustBuilt {
    /// Substring that uniquely identifies the ledger row (matched on the line).
    row_marker: &'static str,
    /// File, relative to the workspace root, that proves the surface exists.
    file: &'static str,
    /// Verbatim substring whose presence in `file` proves the surface exists.
    needle: &'static str,
}

/// Surfaces that exist in code today and therefore must be marked built. Add a
/// row here when you ship something the ledger should never be allowed to call
/// "planned" again.
const MUST_BE_BUILT: &[MustBuilt] = &[
    // The #296 flagship: MCP stdio transport. Proven by the bin subcommand
    // dispatch AND the library entrypoint it calls.
    MustBuilt {
        row_marker: "MCP stdio transport",
        file: "bins/beaterd/src/main.rs",
        needle: "serve_stdio",
    },
    MustBuilt {
        row_marker: "MCP stdio transport",
        file: "crates/beater-mcp/src/lib.rs",
        needle: "pub async fn serve_stdio",
    },
];

/// Planned/deferred ledger rows that legitimately have no tracking issue yet.
/// Matched as a substring against the row line. Keep this list short and
/// justified — every entry is a row we have chosen NOT to require an issue link
/// for. Prefer adding a real `#issue` link in the ledger over growing this.
const PLANNED_WITHOUT_ISSUE_ALLOWLIST: &[&str] = &[
    // Bus/queue adapters — tracked in ARCH §8.4, no standalone issue.
    "NATS JetStream / Kafka bus adapter",
    "Vercel Queue adapter",
    "Object storage (S3/MinIO)",
    "Backend-agnostic migrations",
    // API route groups — each tracked by an ARCH §20.x line item, not an issue.
    "Sessions API",
    "Bulk `promote-from-query`",
    "Mapping importer",
    "Scorers registry",
    "Prompts CRUD",
    "Online evals scores",
    "Registry publish",
    "Native Rust SDK",
    // Ingest / eval / stats planned layers.
    "Session / thread grouping in ingest",
    "Structured message I/O",
    "`beater-scorers`",
    "`beater-online`",
    "Fuzzy match / JSON-schema / embedding scorers",
    "Dataset read APIs",
    "Eval UI",
    "Alert delivery",
    "mSPRT / anytime-valid alerting",
    "Real statistics",
    "`beater-prompts`",
    // RSI / MCP roadmap.
    "RSI improvement tools",
    "RSI anti-overfit guardrail",
    "`beater-studio`",
    "`beater-toolbelt`",
    "`beater-credits`",
    "`beater-mcp-improve`",
    // Replay roadmap.
    "Forked replay / earliest-failing-span attribution",
    // Auth / accounts / security roadmap.
    "Enforced RBAC",
    "`beater-rbac`",
    "`beater-identity`",
    "Storage-layer tenant isolation",
    "Crypto-shred / GDPR deletion",
    "`beater-billing`",
    // Hosting / dashboard roadmap.
    "future thin bin",
    "Docs site",
    "Multi-arch GHCR image",
    "Design system",
    "Client-side query layer",
    "SSE live-tail",
    "`/connect`",
    "`/review`",
    "`/studio`",
    "`/evolution`",
    "`/search`",
    // Bench / governance roadmap.
    "`xtask loadgen`",
    "Compliance docs",
    "Backups / restore drill",
];

/// Parse the `[workspace].members` list from the root `Cargo.toml` and return
/// the set of crate/bin names (the last path segment of each member entry).
fn workspace_member_names(root: &Path) -> anyhow::Result<Vec<String>> {
    let manifest = std::fs::read_to_string(root.join(WORKSPACE_MANIFEST_PATH))
        .with_context(|| format!("read {WORKSPACE_MANIFEST_PATH}"))?;
    let mut names = Vec::new();
    let mut in_members = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("members") && trimmed.contains('[') {
            in_members = true;
            continue;
        }
        if in_members {
            if trimmed.starts_with(']') {
                break;
            }
            // Entries look like `"crates/beater-core",`.
            if let Some(start) = trimmed.find('"') {
                let rest = &trimmed[start + 1..];
                if let Some(end) = rest.find('"') {
                    let path = &rest[..end];
                    if let Some(name) = path.rsplit('/').next() {
                        names.push(name.to_string());
                    }
                }
            }
        }
    }
    if names.is_empty() {
        bail!("parsed zero workspace members from {WORKSPACE_MANIFEST_PATH}; parser is broken");
    }
    Ok(names)
}

/// Return true if the row's status reflects a built/shipped surface (as opposed
/// to planned/deferred). We read the "Claimed status" column (3rd `|`-delimited
/// cell) since that is the field maintainers edit; the verified column tends to
/// carry caveats like "Partial".
fn row_is_built(row: &str) -> bool {
    let cells: Vec<&str> = row.split('|').map(str::trim).collect();
    // | Component | ARCH § | Claimed status | Actual | Notes |
    //   [0 empty]   [1]      [2]      [3]              [4]      [5]
    let claimed = cells.get(3).copied().unwrap_or("");
    let lower = claimed.to_ascii_lowercase();
    lower.contains("built")
}

/// A markdown table data row is a line that starts with `|` and is not the
/// header (`| Component`) or the separator (`|---`).
fn is_data_row(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with('|') && !t.starts_with("|---") && !t.starts_with("| Component")
}

fn check_arch_status() -> anyhow::Result<()> {
    let root = workspace_root()?;
    let ledger = std::fs::read_to_string(root.join(ARCH_STATUS_PATH))
        .with_context(|| format!("read {ARCH_STATUS_PATH}"))?;
    let members = workspace_member_names(&root)?;

    let mut errors: Vec<String> = Vec::new();

    // --- 1. Crate references on BUILT rows must be real workspace members. ---
    // The ledger intentionally names crates that don't exist yet on planned /
    // deferred rows (e.g. `beater-scorers`, `beater-rbac`) — that's the roadmap,
    // not drift. We only enforce membership for rows whose claimed status is
    // built: a row that says "built" but names a `beater-*` crate the workspace
    // doesn't contain is the "references a surface that doesn't exist" failure
    // from #296.
    //
    // We scan only the Component / Claimed / Actual cells (1, 3, 4), NOT the free
    // -form Notes cell (5): a built row commonly references a planned follow-on
    // crate in its notes (e.g. `beater-usage`'s note mentions `beater-billing`
    // (planned)), and that is not drift.
    let crate_re = Regex::new(r"\bbeater-[a-z0-9-]+\b").context("compile crate-name regex")?;
    for (idx, line) in ledger.lines().enumerate() {
        if !is_data_row(line) || !row_is_built(line) {
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let scanned = [cells.get(1), cells.get(3), cells.get(4)]
            .into_iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
            .join(" ");
        for m in crate_re.find_iter(&scanned) {
            let name = m.as_str();
            if !members.iter().any(|member| member == name) {
                errors.push(format!(
                    "line {}: built row references crate `{}` which is not a workspace \
                     member (renamed or never existed?)",
                    idx + 1,
                    name
                ));
            }
        }
    }

    // --- 2. Must-be-built surfaces. -----------------------------------------
    for surface in MUST_BE_BUILT {
        // 2a. The proving artifact must exist and contain the needle.
        let proof_path = root.join(surface.file);
        let proof = std::fs::read_to_string(&proof_path).unwrap_or_default();
        let exists = proof.contains(surface.needle);
        if !exists {
            errors.push(format!(
                "MUST_BE_BUILT misconfigured: `{}` not found in {} (update the checker)",
                surface.needle, surface.file
            ));
            continue;
        }
        // 2b. The ledger row for this surface must be marked built.
        let row = ledger
            .lines()
            .find(|l| is_data_row(l) && l.contains(surface.row_marker));
        match row {
            None => errors.push(format!(
                "no ledger row matches `{}`, but the surface exists in {} \
                 (add/restore the row, marked built)",
                surface.row_marker, surface.file
            )),
            Some(row) if !row_is_built(row) => errors.push(format!(
                "ledger row `{}` is not marked built, but the surface exists in {} \
                 (`{}`). Flip its status to built.",
                surface.row_marker, surface.file, surface.needle
            )),
            Some(_) => {}
        }
    }

    // --- 3. Planned/deferred rows must link an issue or be allowlisted. ------
    let issue_re = Regex::new(r"#[0-9]+|github\.com/").context("compile issue-link regex")?;
    for (idx, line) in ledger.lines().enumerate() {
        if !is_data_row(line) {
            continue;
        }
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        let claimed = cells.get(3).copied().unwrap_or("").to_ascii_lowercase();
        // A row is "planned" for issue-tracking purposes only when it is purely
        // planned/deferred. Rows that are built but mention a planned follow-on
        // (e.g. "built today; NATS/Kafka planned") are NOT untracked-planned rows.
        let is_planned =
            (claimed.contains("planned") || claimed.contains("deferred")) && !row_is_built(line);
        if !is_planned {
            continue;
        }
        let has_link = issue_re.is_match(line);
        let allowlisted = PLANNED_WITHOUT_ISSUE_ALLOWLIST
            .iter()
            .any(|marker| line.contains(marker));
        if !has_link && !allowlisted {
            let component = cells.get(1).copied().unwrap_or("<?>");
            errors.push(format!(
                "line {}: planned/deferred row `{}` has no issue link and is not in the \
                 PLANNED_WITHOUT_ISSUE_ALLOWLIST. Add a `#issue` link or allowlist it.",
                idx + 1,
                component
            ));
        }
    }

    if !errors.is_empty() {
        let mut msg = format!(
            "{} architecture-status drift problem(s) in {}:\n",
            errors.len(),
            ARCH_STATUS_PATH
        );
        for e in &errors {
            msg.push_str("  - ");
            msg.push_str(e);
            msg.push('\n');
        }
        msg.push_str(
            "\nFix the ledger (or update the curated lists in crates/xtask/src/main.rs). \
             See the header of docs/architecture-status.md for the maintainer workflow.",
        );
        bail!(msg);
    }

    println!(
        "no drift: {} crate refs + {} must-built surfaces + planned-row issue links all check out",
        members.len(),
        MUST_BE_BUILT.len()
    );
    Ok(())
}
