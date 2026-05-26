# AGENTS.md

Guidance for AI coding agents (GitHub Copilot, Claude Code, etc.) working in
this repository. Human contributors are welcome to read it too — it is a
superset of `.github/copilot-instructions.md` and reflects the actual layout,
conventions, and CI of this crate.

If anything here conflicts with `CONTRIBUTING.md` or the repository owners'
direction, the human guidance wins. Flag the contradiction in your PR
description rather than silently picking one.

---

## 1. What this repository is

`npcx490m-pac` is a **Peripheral Access Crate (PAC)** for the Nuvoton
**NPCX490M** family of embedded controllers. It is consumed by higher-level
HAL crates (e.g. an Embassy-based HAL) and by application firmware.

Key facts:

- Crate name: `npcx490m-pac` (see `Cargo.toml`).
- Edition: `2021`. MSRV: **1.80** (enforced in CI; do not raise without
  updating `Cargo.toml` *and* both `check.yml` and `rolling.yml`).
- License: MIT.
- `#![no_std]` library targeting Cortex-M (NPCX490M is ARMv7E-M / Cortex-M4F;
  the no-std CI target is `thumbv8m.main-none-eabihf`).
- Almost all Rust source under `src/` is **generated** by
  [`svd2rust`](https://github.com/rust-embedded/svd2rust) from a patched SVD
  file. See §3 for the regeneration pipeline.

### Features

Defined in `Cargo.toml`:

| Feature  | Effect                                                                 |
|----------|------------------------------------------------------------------------|
| `rt`     | Enables `cortex-m-rt/device`; `build.rs` emits `device.x` so the linker can place vectors. |
| `defmt`  | Adds `defmt::Format` impls on generated register types.                |
| `debug`  | Adds `core::fmt::Debug` impls on generated register types (gated to keep release builds small). |

Dependencies are intentionally minimal: `cortex-m`, `cortex-m-rt` (optional),
`critical-section` (optional), `defmt` (optional), `vcell`. Do not add new
runtime dependencies without strong justification — PACs should stay light.

---

## 2. Repository layout

```
.
├── AGENTS.md                       # this file
├── Cargo.toml / Cargo.lock         # crate manifest + locked deps
├── CODEOWNERS / CODE_OF_CONDUCT.md / CONTRIBUTING.md / SECURITY.md / LICENSE
├── build.rs                        # copies `device.x` into OUT_DIR when `rt` is enabled
├── device.x                        # linker fragment naming interrupt vectors
├── deny.toml                       # cargo-deny config (licenses, advisories, bans, sources)
├── generate.sh                     # one-shot regeneration script (Linux/macOS)
├── npcx490m-chip.yaml              # chip family description (probe-rs style)
├── patch/                          # svdtools YAML patches applied to svd/NPCX490M.svd
│   ├── NPCX490M.yaml               # top-level patch entry point (_include's the per-peripheral files)
│   └── <peripheral>.yaml           # per-peripheral fix-ups (adc, espi, gpio, i3c, smb, …)
├── src/                            # GENERATED — do not hand-edit (see §3)
│   ├── lib.rs                      # entry point produced by svd2rust + form
│   ├── generic.rs                  # common register/field machinery
│   └── <peripheral>/…              # one module per peripheral
├── supply-chain/                   # cargo-vet audits and config
├── svd/NPCX490M.svd                # vendor SVD (source of truth for register layout)
└── .github/
    ├── copilot-instructions.md     # commit-message + Assisted-by trailer rules
    └── workflows/                  # CI: check, nostd, rolling, cargo-vet, cargo-vet-pr-comment
```

---

## 3. The source of truth is the SVD, not `src/`

This is the most important rule for any agent working here.

1. **`svd/NPCX490M.svd` is the source of truth** for what registers, fields,
   and enums exist. The vendor file is imperfect, so it is patched.
2. **`patch/NPCX490M.yaml`** is the top-level
   [`svdtools`](https://github.com/rust-embedded/svdtools) patch. It includes
   one YAML per peripheral under `patch/*.yaml`. *All* register-layout fixes
   live here.
3. **`src/**/*.rs` is generated** from the patched SVD by `svd2rust`, then
   split into modules by [`form`](https://crates.io/crates/form), then
   formatted by `cargo fmt`.
4. Therefore, **never hand-edit files under `src/`**. Any local edit will be
   wiped out the next time someone regenerates the crate, and reviewers
   should reject such diffs. If a generated file is wrong, fix the SVD patch
   or report the bug upstream against `svd2rust`/`svdtools`.

### Regeneration pipeline

The authoritative recipe lives in `README.md` and (for Linux/macOS) in
`generate.sh`:

```bash
svdtools patch patch/NPCX490M.yaml
svd2rust -i svd/NPCX490M.svd.patched \
    --reexport-interrupt --reexport-core-peripherals \
    --ignore-groups --impl-defmt defmt \
    --impl-debug --impl-debug-feature debug
rm -r src/*
form -i lib.rs -o src
rm lib.rs
cargo fmt
```

On Windows, replace `/` with `\` and run `dos2unix` on the generated files
under `src/` so line endings match the rest of the tree (the repository is
LF-only — see §6).

### What an agent should and shouldn't do here

- ✅ Add or modify YAML in `patch/` to correct a register, field, enum value,
  reset value, access type, or peripheral instance count.
- ✅ Update the SVD (`svd/NPCX490M.svd`) only if you have an authoritative
  vendor update; prefer a patch over editing the SVD directly so the upgrade
  path stays clean.
- ✅ Regenerate `src/` after any patch/SVD change and commit the regeneration
  in the **same commit** (or an immediately following commit with a clear
  message) so the tree always builds.
- ❌ Edit anything under `src/` directly — including `src/lib.rs`,
  `src/generic.rs`, and the per-peripheral modules.
- ❌ Add `// TODO`/manual workarounds inside generated code.
- ❌ Hand-tune formatting in generated files; rely on `cargo fmt` and the
  generator.

If you are unsure whether a file is generated, assume it is if it lives under
`src/` and was produced by `form` (top-of-file docstring referencing
`svd2rust` is a strong signal — see `src/lib.rs`).

---

## 4. Build, test, and check commands

These mirror what CI runs. Run them locally before claiming a change is
complete.

### Primary checks (from `.github/workflows/check.yml`)

```bash
cargo fmt --check                       # formatting (stable)
cargo clippy                            # lints (stable AND beta in CI)
cargo doc --no-deps --all-features      # docs (nightly in CI, RUSTDOCFLAGS=--cfg docsrs)
cargo hack --feature-powerset check     # every combination of features
cargo deny --all-features check         # licenses/advisories/sources/bans
cargo check                             # MSRV build, toolchain 1.80
```

### no-std (from `.github/workflows/nostd.yml`)

```bash
rustup target add thumbv8m.main-none-eabihf
cargo check --target thumbv8m.main-none-eabihf --no-default-features
```

### Rolling / nightly drift (from `.github/workflows/rolling.yml`)

```bash
cargo update
cargo hack --feature-powerset check
cargo +1.80 check --all-features
```

### Supply chain (from `.github/workflows/cargo-vet.yml`)

```bash
cargo vet --locked
```

If you add or bump a dependency, expect `cargo vet` to complain. Run
`cargo vet suggest` and either add an audit, an exemption in
`supply-chain/config.toml`, or wait for a maintainer to vet it. Do **not**
disable the workflow.

### Local convenience

There is no `Makefile` or `xtask`. The minimal "did I break it?" loop is:

```bash
cargo fmt --check && cargo clippy --all-features -- -D warnings && \
  cargo hack --feature-powerset check && \
  cargo check --target thumbv8m.main-none-eabihf --no-default-features
```

There are no unit tests in this crate (the generated code has no runtime
behavior to test in isolation); `cargo test` is effectively a build check.

---

## 5. Coding conventions

Because almost everything is generated, the surface area for "style" is
small. The rules that do apply:

- **`rustfmt` defaults.** There is no `rustfmt.toml`; do not introduce one
  unless the maintainers agree to a new style baseline.
- **Clippy clean on stable and beta.** CI fails on new clippy warnings via
  `clippy-action`. If clippy flags generated code, fix the *generator input*
  (patch, svd2rust version) rather than sprinkling `#[allow(...)]` in
  `src/`.
- **No new runtime dependencies** without discussion. PACs must stay
  lightweight and dependable.
- **Features stay additive.** `cargo hack --feature-powerset check` will
  detect non-additive features.
- **No `unsafe` outside what `svd2rust` already emits.** The generated code
  contains the only `unsafe` this crate needs.
- **No `std`.** Everything is `#![no_std]`. Don't pull in `std` even for
  tests or examples.

For hand-written files (`build.rs`, `generate.sh`, YAML patches): keep them
minimal, deterministic, and free of host-specific assumptions where
possible.

---

## 6. Line endings

The tree is LF-only. The `README.md` regeneration steps explicitly call out
running `dos2unix` after Windows generation. Agents working on Windows must:

```bash
git config core.autocrlf false
```

before staging changes, and must not introduce CRLF (`^M`) in any file. Run
`git diff --check` before committing — it will flag whitespace and EOL
problems.

There is no `.gitattributes` enforcing this today; treat LF as the
convention until one is added.

---

## 7. Commit messages and AI attribution

These rules are the canonical content of `.github/copilot-instructions.md`
and apply to **every** commit, AI-assisted or not.

### Commit message format

- Subject line: capitalized, **50 characters or fewer**, imperative mood
  (e.g. "Fix bug", not "Fixed bug" or "Fixes bug").
- Separate subject from body with a single blank line.
- Wrap the body at **72 characters**.
- Use the body to explain *what* and *why*, not *how*.

### Assisted-by trailer (mandatory for AI-assisted commits)

Every commit that includes AI-generated or AI-assisted work **must**
contain an `Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

Where:

- `AGENT_NAME` is the AI tool/framework (e.g. `GitHub Copilot`).
- `MODEL_VERSION` is the specific model used (e.g. `claude-opus-4.7`).
- Optional bracketed tokens name specialized analysis tools used
  (e.g. `coccinelle`, `sparse`, `smatch`, `clang-tidy`). Basic development
  tools (git, cargo, editors) are **not** listed.

Rules:

- AI agents **MUST verify their own identity** (agent + model version)
  before composing the trailer. Do not hard-code a model name from a
  previous session.
- AI agents **MUST NOT** add a `Signed-off-by` trailer. Only humans can
  certify the Developer Certificate of Origin.
- Keep the trailer block at the very end of the commit message, after a
  blank line, in standard git trailer format.

### Authorship

Commits authored by AI agents on behalf of a human should set
`user.name`/`user.email` to that human per-invocation (e.g.
`git -c user.name="..." -c user.email="..." commit ...`) rather than
mutating the global git config.

---

## 8. Working with pull requests

- Branch off the latest `upstream/main`. The default branch is `main`.
- Keep PRs focused: a SVD/patch fix + the regenerated `src/` is fine in one
  PR; mixing unrelated peripheral patches makes review hard.
- Run the full local check loop in §4 before pushing.
- Reference the upstream peripheral/datasheet section in the PR description
  whenever you change `patch/*.yaml`, so a reviewer can verify the fix.
- Do not force-push shared branches; rebase locally and push to your fork.

`CONTRIBUTING.md` covers the licensing terms — contributions are MIT, and
if you wish to contribute code authored by others or under another license
you must say so in the PR.

---

## 9. Things that commonly go wrong (and how to avoid them)

- **Editing `src/` directly.** Will be overwritten on next regen. Fix the
  patch instead. See §3.
- **MSRV creep.** The crate must build on Rust 1.80. Don't use newer-stdlib
  APIs or syntax in the (small) hand-written code. CI's `msrv` job will
  catch this.
- **Non-additive features.** `cargo hack --feature-powerset check` will
  fail. Make sure new feature flags only add capability, never remove or
  alter existing API.
- **Forgetting `device.x`.** The `rt` feature requires `build.rs` to copy
  `device.x` into `OUT_DIR`. If you rename interrupt symbols (via a SVD
  patch), keep `device.x` in sync.
- **Pulling in `std`.** Anything that breaks `cargo check --target
  thumbv8m.main-none-eabihf --no-default-features` will fail the no-std
  job.
- **New dependencies without vetting.** `cargo vet --locked` will fail in
  CI. Add an audit or exemption in `supply-chain/`.
- **Windows line endings.** See §6. Run `dos2unix` after Windows
  regeneration and verify with `git diff --check`.
- **Bumping `svd2rust` without regenerating everything.** Always regenerate
  the full `src/` tree from a clean state (`rm -r src/*` per the recipe) so
  diffs are honest.

---

## 10. Where to look first

| If you want to…                                  | Start here                                    |
|--------------------------------------------------|-----------------------------------------------|
| Fix a wrong register/field                       | `patch/<peripheral>.yaml`, then regenerate    |
| Add a missing peripheral                         | `svd/NPCX490M.svd` + new `patch/<peri>.yaml`  |
| Change the interrupt vector table                | `device.x` + relevant SVD `<interrupt>`s      |
| Adjust feature flags                             | `Cargo.toml` + `check.yml`                    |
| Update MSRV                                      | `Cargo.toml` (`rust-version`), `check.yml`, `rolling.yml` |
| Tweak CI                                         | `.github/workflows/*.yml`                     |
| Audit a new dependency                           | `cargo vet`, then `supply-chain/`             |
| Understand the chip variants                     | `npcx490m-chip.yaml`                          |

When in doubt: read `README.md`, then `generate.sh`, then the relevant
workflow under `.github/workflows/`. Those three files are the closest
thing this crate has to executable documentation.

## Model selection & cost discipline

Premium models (Opus, GPT-5 family, "high"/"xhigh" reasoning variants)
cost an order of magnitude more than standard models (Sonnet, Haiku,
mini). Most steps in a typical task do not need premium reasoning,
and over-using premium models wastes credits without improving
outcomes. The rules below apply to *all* model selection: your own
session, sub-agents launched via the `task` tool, and parallel work
launched via `/fleet`.

### Default posture

- **Default to the cheapest model that can do the job.** Reach for a
  premium model only when one of the escalation triggers below is hit.
- **Plan with premium, execute with cheap.** Spend at most one or two
  premium turns on design / planning, then downshift to a cheaper
  model for mechanical execution of the plan.
- **Never bump the model "just in case."** If you cannot articulate
  *why* a cheaper model would fail, use the cheaper model.

### Escalation triggers (use a premium model)

Reach for a premium model when *any* of these are true:

- Cross-module refactor, architectural design, or API design from
  scratch.
- Subtle correctness reasoning: concurrency, lifetimes, `unsafe`,
  FFI ABI, cryptography, safety-critical control paths.
- Debugging a failure that survived one prior cheap-model attempt.
- Reviewing code on a safety-, security-, or money-critical path.
- The diff cannot be predicted in advance — i.e. there is genuine
  creative or design work to do, not just typing.

### De-escalation triggers (use a cheap model)

Use the cheapest available model when *any* of these are true:

- Searching, reading, summarising files or docs.
- Single-file mechanical edits: rename, format, lint fix, dependency
  bump, boilerplate, scaffolding from a known template.
- Generating tests for code that already works.
- Running builds, tests, linters, or other commands where the model
  only needs to report success/failure.
- Routine commits, PR descriptions, changelog entries.
- The diff is essentially predictable before generation.

### Sub-agent routing (the `task` tool)

When delegating with the `task` tool, set `model:` explicitly. Do not
let sub-agents inherit a premium default for cheap work.

| Sub-agent type    | Default model             | Override to                                     |
|-------------------|---------------------------|-------------------------------------------------|
| `explore`         | cheap                     | keep cheap (`claude-haiku-4.5` or `gpt-5-mini`) |
| `task` (run cmd)  | cheap                     | keep cheap                                      |
| `research`        | cheap for breadth         | premium only for the final synthesis            |
| `general-purpose` | match task                | cheap for mechanical work; premium for design   |
| `rubber-duck`     | premium                   | keep premium — this is where reasoning pays off |
| `code-review`     | premium on critical paths | cheap on cosmetic / mechanical diffs            |

### `/fleet` (parallel sub-agents) rules

- Fleet mode multiplies cost by the fleet width. Apply the rules
  above *per worker*, not in aggregate.
- Split a fleet job along complexity lines: route the cheap,
  parallelisable workers (file edits, test runs, doc updates) to a
  cheap model; reserve premium models for the small number of
  workers that need real reasoning.
- If every worker in a fleet would need a premium model, the work is
  probably not a good fit for fleet mode — reconsider the
  decomposition before paying N× premium.

### Session hygiene

- Keep sessions short and focused. Long premium sessions are the
  single largest source of waste because every turn re-processes the
  full history.
- Use `/compact` when the conversation grows long, and `/new` for
  unrelated work.
- Prefer `/ask` for one-off side questions so they don't extend the
  main session.

### When in doubt

Ask: *"If a cheaper model produced the wrong answer here, would I
catch it in seconds (compiler, tests, my own review) or in
weeks (production incident)?"* If the former, use the cheap model
and let the feedback loop do its job.
