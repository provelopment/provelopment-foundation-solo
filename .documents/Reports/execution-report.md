# Foundation Solo — Execution Report

> **One canonical report.** Overwritten in place every run; Git history retains earlier revisions. It is not a WIP file, grants no authority, and its own commit SHA appears only in the chat handback.

## Run identity

| Field | Value |
| --- | --- |
| Date | 2026-11-07 |
| Work order | **`SOLO-WO-026` edition `R03`** — Stage 01 `R01.A` minimal reproducible engineering foundation |
| Owner handoff | Received — instruction to align with `README.md`/`AGENTS.md` and execute the installed `SOLO-WO-026 R03` |
| Local root | `C:\Users\Sean\Documents\Provelopment\00.foundation-solo` (verified, single checkout) |
| Initial source HEAD / tree | `ff658baaaa52d8eeae9a1414c976b9597bf5b459` / `5421891e2078eb5bc7b815adf6b88949e40491d9` |
| Implementation commit (local) | **`d3df20af4c76ebd734cf0b0b8118e2e401b3373b`** / tree **`71bf0196f7a28065b422effdf0dafddaf56a6e6a`** |
| **Status** | **`STOPPED`** — one mandatory gate (A1-07 byte-identical reproducibility) failed on this host; see the STOP section |

## A1-01 Local safety and scope — `PASS`

The actual root is the existing Solo checkout; branch `main`; entry HEAD equal to `ls-remote`; exactly one worktree and one checkout; sole `origin` = `provelopment/provelopment-foundation-solo`; no stashes; no reparse-point entries; four and only four WIP files; `.temp/` empty at entry (ignored). The only pre-existing change was the Owner's replacement of the neutral ACTIVE file with this order — no other dirty or untracked path existed. All **fourteen** reviewed baseline blobs matched: `README.md` `3f3005d6`, `AGENTS.md` `32562b67`, About `1d80f627`, Architecture `03e8923f`, `sop-architect` `8760be32`, `sop-implementation` `a5771160`, ROADMAP `4f46dfcc`, NEXT `3b753df8`, pre-installation ACTIVE `ea5f20b7`, OUTLOOK `cb9c7a61`, report `870e9064`, SECURITY `dd321856`, LICENSE `d6456956`, `.gitignore` `f51cc530`. `R01.A` was the sole NEXT candidate and `R01.A`/`R01.B`/`R01.C` remained in ROADMAP. All changed paths fall inside the §6 allow-list; no `target/` directory exists in the repository (all build output was directed to `.temp/`).

## WIP transition — the express §3 issue transition

`.documents/WIP/plan-next.md` now contains exactly `# Foundation Solo — plan-next (Next Candidate)` plus one LF (**49 bytes**, verified byte-wise, no BOM). ROADMAP, OUTLOOK and the ACTIVE scope were not touched; no successor candidate was installed; no acceptance or closure is claimed. `R01.A`, `R01.B` and `R01.C` remain in ROADMAP.

## A1-02 Workspace and toolchain — `PASS`

Four real Rust 2024 crates, resolver 3, one tracked root `Cargo.lock`, `rust-toolchain.toml` with `channel = "1.99.1"` and `rustfmt`/`clippy`. `cargo tree --workspace --all-features --locked` (exit 0) shows exactly: `solo-adapters → solo-application → solo-domain`, `solo-adapters → solo-domain`, `solo-application → solo-domain`, `solo-domain` **(no dependencies)**, and `solo-cli → solo-adapters, solo-application` with `[dev-dependencies] serde_json v1.0.154 → itoa v1.0.18, memchr v2.8.3, serde_core v1.0.229, zmij v1.0.23`. Locked set: **11 packages**, one direct external crate only. `#![forbid(unsafe_code)]` is set at the root of all four crates.

**Environment deviation, disclosed.** No Rust toolchain existed on this machine and no MSVC linker or Windows SDK is installed, so the channel's default MSVC host could not link. I installed exactly the pinned release from official `static.rust-lang.org` into task-owned scratch and used its **`x86_64-pc-windows-gnu`** host variant: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, `rustfmt 1.10.0-stable`, `clippy 0.1.99`, host `x86_64-pc-windows-gnu`, linker `rust-mingw`. The compiler version and commit are the pinned ones; the host triple is not, and `RUSTUP_TOOLCHAIN` selected it because the committed `rust-toolchain.toml` correctly pins the channel and would otherwise resolve the MSVC host. **The MSVC-hosted build was never tested here.**

## A1-03 Identity — `PASS`

`EntityIdV1` implements `^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$` over 1–63 ASCII bytes with no coercion. Eleven tests cover acceptance (`solo`, `entity-7`), both length boundaries (1 and 63 bytes), and refusal of: empty, 64 bytes, uppercase (`Main`, `ENTITY`, `Solo`), leading digit/underscore/dot/hyphen (`1x`, `_a`, `.a`, `-abc`), internal forbidden characters (`a.b`, `a/b`, `a%2Fb`, space, tab, `+`, `:`), non-ASCII (`münchen`, `ソロ`, en dash), a trailing hyphen and doubled hyphens. Diagnostics are fixed ASCII tokens and never echo authored input.

## A1-04 Base candidate — `PASS`

`BaseCandidateV1` accepts lowercase `http://`/`https://` with an ASCII DNS host (≤253 octets, labels 1–63, edge-alphanumeric), optional port 1–65535 with default-port removal, and 0–5 prefix segments `[a-z][a-z0-9-]*`, canonicalizing one terminal slash. Thirteen tests cover: the required `https://EXAMPLE.com:443/alpha → https://example.com/alpha`, a cleartext origin, origin-only slash collapse, terminal-slash removal, default-port removal, nondefault-port preservation, the five-segment maximum and idempotent canonicalization; twenty-nine refusals covering scheme and scheme case, host and label shape, port (`:0`, `:65536`, `:080`, `:8a`), empty and dot segments, uppercase or otherwise invalid segments, percent-encoding, credentials, query, fragment, backslash and control characters; and five `unsupported-by-this-preflight` refusals (Unicode host, `xn--` label, three IP-literal forms). A test asserts refusal text never echoes authored input or control sequences.

## A1-05 CLI and use case — `PASS`

Real binary → adapter → application → domain with no persistent effect. Observed on the compiled process: `--entity-id entity-7 --base-url https://EXAMPLE.com:443/alpha` → exit `0`, single stdout line `preflight=ok entity_id=entity-7 base=https://example.com/alpha`, empty stderr; `--entity-id Main` → exit `2`, empty stdout, `error: PREFLIGHT_ENTITY_ID_INVALID: invalid-start`; `http://example.org` origin → exit `0`; five prefix segments and `:8443` preserved; reversed flag order accepted; identity unchanged when the base becomes `https://other.example/beta`. Refusals proven: missing, duplicate, unknown flag, positional token, unknown subcommand, missing value, non-UTF-8 argument (`PREFLIGHT_ARGS_ENCODING`), `PREFLIGHT_BASE_INVALID`, `PREFLIGHT_BASE_UNSUPPORTED` — all exit `2` with no stdout. One process test runs the binary inside an empty scratch directory and asserts it creates no file. Exit `1` is reserved for internal error and is unreachable through authored input, so it is verified at the mapping level only.

## A1-06 Hexagonal dependency — `PASS`

`crates/solo-cli/tests/workspace_graph.rs` reads **real** `cargo metadata --locked --all-features` output (13 tests). It builds a graph from declared edges (true package names, dependency kind, rename recorded separately) plus resolved edges, then rejects: any `solo-domain` edge, `solo-application` beyond `solo-domain`, `solo-adapters` beyond domain/application, and any `solo-cli` external dependency other than the approved dev-dependency. It also detects workspace cycles and transitive external reachability from the three inner crates. Negative fixtures prove rejection of `solo-domain → solo-adapters`, `solo-application → solo-cli`, `solo-application → solo-adapters`, `solo-domain → external`, a **renamed** illegal edge, illegal **dev** and **build** edges, and a cycle. A dedicated test shows a disabled guard accepts the very graph the negatives reject, so removing or bypassing the guard fails this suite. The real workspace passes, contains exactly the four required crates, and declares only the approved external dependency. The full source-level checker (aliases, re-exports, macros, `include!`, qualified paths, dynamic loading) is **deferred to `R01.B`** and is not claimed here.

## A1-07 Quality and reproducibility — `FAILED` (partial)

Green, with actual exit codes: `cargo fmt --all -- --check` → `0`; `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` → `0`; `cargo test --workspace --all-targets --locked` → `0` (**66 tests**: 11 identity, 13 base, 9 application, 11 adapter, 9 CLI process, 13 graph); `serde_json = "=1.0.154"` is a `solo-cli` dev-dependency only and appears in no production path. `cargo build --workspace --release --locked` → `0`; `cargo metadata --format-version 1 --locked --all-features` → `0`; `cargo tree --workspace --all-features --locked` → `0`; `cargo test --workspace --all-targets --locked --offline` → `0` once the cache was populated; a `--locked --offline` build with **no** lock file refuses with `error: cannot create the lock file … because --locked was passed`, exit `101`. `cargo-deny` is **`NOT RUN`** — no `cargo-deny` executable exists in this environment; dependency risk was assessed manually (A1-08) and no scan is claimed.

**The failed half.** Two independent, fresh, task-owned Cargo caches and target directories were built against the same checkout with the same compiler, target and features. The shipped `solo-cli.exe` hashes **differ**: `E71E71729B1E7C9217D5BFF4CFFB0FB30D9EBE58BA90B32A3BCFAD65C20222A4` versus `D8BBFD1123CB5725CE5936C5F58A062DD43C697C532FEE11DD6740BB431D72F4`. Both are 1,242,444 bytes and differ in exactly **two bytes**: offset **136**, the PE COFF `TimeDateStamp`, and offset **216**, the PE `CheckSum`, which covers that timestamp — the GNU linker stamping wall-clock build time. Cause isolated by experiment: with `-C link-arg=-Wl,--no-insert-timestamp`, two further independent fresh-cache builds are **byte-identical** (`2169CED0D3D1EE68F5A5F17F25690A1C21A21CF8C7D69140B939466C23F097E0`), so code, layout and lockfile are deterministic and only the timestamped header fields vary. Under the order's wording — *"a mismatch is a failed gate, not an acceptable approximation"* — the gate is **FAILED** as commanded, and I did not reclassify it.

## A1-08 Security and supply chain — `PASS` (with `NOT RUN` items)

No secrets, credentials or tokens exist in the change; no environment variable, file, current directory, hostname or request supplies an authored value, and there is no fallback, prompt or inference path. Diagnostics are composed only from fixed ASCII tokens for codes and details; a test asserts no control character and no echo of authored input reaches stderr, and that stdout stays empty on refusal. No `unsafe` is permitted in any crate, the domain is pure, nothing is logged, and no network, filesystem or process-spawning code exists in the shipped binary. Provenance of the five external packages was reviewed against crates.io: `serde_json 1.0.154` (MIT OR Apache-2.0, serde-rs), `itoa 1.0.18` (MIT OR Apache-2.0, dtolnay), `memchr 2.8.3` (Unlicense OR MIT, BurntSushi), `serde_core 1.0.229` (MIT OR Apache-2.0, serde-rs), `zmij 1.0.23` (MIT, dtolnay, serde_json's float-formatting dependency); none is yanked and all licences are compatible with this repository's Apache-2.0. No git or floating dependency is used. `cargo-deny` remains `NOT RUN`; no advisory-database scan was performed here.

## A1-09 Local completion and publication — `PASS` for the local half

One fact-based report (this file), a clean tree at handback, a single final push of the authorized commits and successful push confirmation are the intended close. `R01.B`, `R01.C`, Stage 02+ and all provider/release work were **not** started. ROADMAP and ACTIVE are unchanged; NEXT holds only the neutral heading; nothing was self-accepted, merged, released or deployed.

## STOP — blocker and smallest decision

**Blocker.** A1-07's byte-identical reproducibility requirement cannot be satisfied by the commanded build on this host, because the GNU-hosted linker stamps a wall-clock `TimeDateStamp` (and the derived PE `CheckSum`) into the executable. The compiled code itself is deterministic, as the timestamp-suppressed experiment proves.

**Why I stopped instead of working around it.** A permanent fix needs a link flag (`-Wl,--no-insert-timestamp`), which could only be committed through a path this order does not authorize (for example `.cargo/config.toml`). Suppressing it only for the verification runs would leave the committed default build irreproducible while letting the gate read `PASS` — precisely the silent narrowing the order forbids. The MSVC host, the variant the committed `rust-toolchain.toml` would normally resolve, could not be tested, so I also cannot claim the failure is inherent to the pinned channel rather than to the GNU host variant this machine forced.

**Smallest necessary Owner/Architect decision — one of:**

1. Authorize a committed reproducibility mechanism in a successor order (a new allow-listed path such as `.cargo/config.toml` carrying `rustflags = ["-C", "link-arg=-Wl,--no-insert-timestamp"]` for this target), keeping A1-07 as strict byte-identity.
2. Provide or authorize an environment with the MSVC build tools so the pinned channel builds as authored, then re-run A1-07 there.
3. Restate A1-07 to compare binaries after normalizing the PE timestamp and checksum fields, recording that normalization as part of the evidence.

The work completed so far is committed locally and remains reviewable; none of it is a completion claim for `R01.A`, and no `ACCEPT` is asserted on the Owner's or Architect's behalf.

## Deferred scope, limits and cleanup

Deferred and **not** passed: `C1`–`C4`; `F01`–`F17`; `RG01`–`RG11`; the `R01.B` source-level boundary checker; `R01.C` CI, HTTP/browser smoke and real request-to-application integration; Node/npm/TypeScript/Vite/Playwright; Stage 02+ publication, routing, site, locale and visibility; WCAG/browser/CWV measurement; provider deployment; `NOTICE`/SBOM; release. Nothing here is a product release, a deployment or an acceptance.

Line endings: every new index entry is stored as **LF** (`git ls-files --eol` reports `i/lf` throughout, per the repository's own `* text=auto` policy); local Windows working copies may be CRLF and `.gitattributes` was not edited.

Cleanup: all build output, the task-owned Cargo caches, the smoke crate, the lock-refusal probe and scratch metadata live under Solo `.temp/`, which is ignored and removed at run end; the OS-temp lock probe was deleted and verified gone. Unknown or Owner-owned material was never touched, and no destructive Git operation was performed.



