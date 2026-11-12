//! Load-bearing guard: `unsafe` is confined to two exact, platform-gated native modules.
//!
//! The architecture allows exactly two project-owned `unsafe` regions — the Windows source
//! reader must call `NtCreateFile` directly, and the path
//! `crates/solo-adapters/src/media_worker_host_native.rs` reserved by `SOLO-WO-074 R01` now holds
//! the Linux-only limit **readback** seam added by `SOLO-WO-076 R01`, which observes two
//! `getrlimit` ceilings and launches nothing. Everywhere
//! else the code is safe Rust. A comment in a crate header cannot enforce that, so this file does
//! it mechanically:
//!
//! * every first-party `.rs` file is scanned after comments and string literals are removed, so a
//!   *mention* of the word never counts as a use of it;
//! * only those two exact paths may use the keyword at all;
//! * whether an occurrence is genuinely inside a **positive** platform gate is decided by the
//!   architecture checker's own `syn` AST rule, not by substring matching, so a negated,
//!   `any`-alternated, malformed or merely textual `windows`/`linux` mention never counts;
//! * every `unsafe` block in an eligible file must carry an adjacent `// SAFETY:` rationale;
//! * the adapter's lint arrangement must be exactly `#![deny(unsafe_code)]` plus two
//!   module-scoped allowances — `#[cfg_attr(windows, allow(unsafe_code))]` on the existing
//!   source reader and `#[cfg_attr(target_os = "linux", allow(unsafe_code))]` on the Linux
//!   readback seam — with no third, wildcard, combined, negated or crate-wide one;
//! * the native roots each eligible path may name are fixed: the source reader may use
//!   `windows_sys`/`rustix`, the reserved process path may use `windows_sys`/`libc`, and every
//!   other file may use none of them — `libloading`, `dlopen` and `LoadLibrary` never.
//!
//! The final tests feed deliberately bad sources through the same functions, so the guard is
//! proven to refuse rather than merely to pass.

mod architecture_checks;

use std::fs;
use std::path::{Path, PathBuf};

/// The pre-existing file permitted to contain `unsafe` and to import its native roots.
const NATIVE_MODULE: &str = "crates/solo-adapters/src/source_read_native.rs";

/// The reserved process-native path (`SOLO-WO-074 R01`), filled by `SOLO-WO-076 R01` with a
/// Linux-only limit-readback seam.
const RESERVED_PROCESS_MODULE: &str = "crates/solo-adapters/src/media_worker_host_native.rs";

/// Which exception, if any, one repository-relative path may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    /// No native exception: no `unsafe` and no native root.
    Ordinary,
    /// The Windows-only source reader.
    SourceRead,
    /// The reserved Linux/Windows process-control boundary.
    ProcessNative,
}

/// The role of one repository-relative path.
fn role_of(path: &str) -> Role {
    if path == NATIVE_MODULE {
        Role::SourceRead
    } else if path == RESERVED_PROCESS_MODULE {
        Role::ProcessNative
    } else {
        Role::Ordinary
    }
}

/// The native roots the source reader may name: the audited Windows and Linux source-read crates.
const SOURCE_READ_ROOTS: [&str; 2] = ["windows_sys", "rustix"];

/// The native roots the reserved process boundary may name: process control only.
const PROCESS_NATIVE_ROOTS: [&str; 2] = ["windows_sys", "libc"];

/// Roots that would reach a dynamically loaded object, or an unapproved native API, from any
/// file. `libc` is here on purpose: it is not a general-purpose door, and the only file that may
/// name it is the reserved process boundary above.
const FORBIDDEN_ROOTS: [&str; 5] = ["libloading", "libdl", "dlopen", "dlsym", "LoadLibrary"];

/// Every native root this guard knows about, so an unexpected one is refused rather than ignored.
const ALL_NATIVE_ROOTS: [&str; 8] = [
    "windows_sys",
    "rustix",
    "libc",
    "libloading",
    "libdl",
    "dlopen",
    "dlsym",
    "LoadLibrary",
];

/// The adapter crate root that must carry the approved lint arrangement.
const ADAPTER_ROOT: &str = "crates/solo-adapters/src/lib.rs";

/// The repository root, two levels above this crate's manifest directory.
fn repository_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .expect("crates/solo-cli has a repository root")
        .to_path_buf()
}

/// Every first-party `.rs` file that ships or tests this workspace.
fn first_party_sources(root: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(entries) = fs::read_dir(root.join("crates")) else {
        return found;
    };
    for entry in entries.flatten() {
        for directory in ["src", "tests"] {
            collect(&entry.path().join(directory), &mut found);
        }
    }
    found.sort();
    found
}

fn collect(directory: &Path, found: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            found.push(path);
        }
    }
}

/// The repository-relative spelling of an absolute path, for stable diagnostics.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

/// Removes comments, string literals and character literals.
///
/// After this the word `unsafe` can only appear as an actual keyword, an attribute or an
/// identifier. Lifetimes such as `&'a str` survive as ordinary code, so the scanner cannot be
/// tricked into swallowing a whole function body.
fn code_only(source: &str) -> String {
    let characters: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut index = 0;
    while index < characters.len() {
        let current = characters[index];
        let next = characters.get(index + 1).copied();

        if current == '/' && next == Some('/') {
            while index < characters.len() && characters[index] != '\n' {
                index += 1;
            }
            continue;
        }

        if current == '/' && next == Some('*') {
            let mut depth = 0_usize;
            while index < characters.len() {
                if characters[index] == '/' && characters.get(index + 1) == Some(&'*') {
                    depth += 1;
                    index += 2;
                    continue;
                }
                if characters[index] == '*' && characters.get(index + 1) == Some(&'/') {
                    index += 2;
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                    continue;
                }
                index += 1;
            }
            out.push(' ');
            continue;
        }

        if current == 'r' || (current == 'b' && next == Some('r')) {
            let mut cursor = if current == 'b' { index + 1 } else { index };
            cursor += 1;
            let mut hashes = 0_usize;
            while characters.get(cursor) == Some(&'#') {
                hashes += 1;
                cursor += 1;
            }
            if characters.get(cursor) == Some(&'"') {
                cursor += 1;
                while cursor < characters.len() {
                    let closes = characters[cursor] == '"'
                        && (0..hashes)
                            .all(|offset| characters.get(cursor + 1 + offset) == Some(&'#'));
                    if closes {
                        cursor += 1 + hashes;
                        break;
                    }
                    cursor += 1;
                }
                out.push(' ');
                index = cursor;
                continue;
            }
            out.push(current);
            index += 1;
            continue;
        }

        if current == '"' {
            index += 1;
            while index < characters.len() {
                if characters[index] == '\\' {
                    index += 2;
                    continue;
                }
                if characters[index] == '"' {
                    index += 1;
                    break;
                }
                index += 1;
            }
            out.push(' ');
            continue;
        }

        if current == '\'' {
            let closes = characters.get(index + 1) == Some(&'\\')
                || matches!(
                    (characters.get(index + 1), characters.get(index + 2)),
                    (Some(_), Some('\''))
                );
            if closes {
                index += 1;
                while index < characters.len() {
                    if characters[index] == '\\' {
                        index += 2;
                        continue;
                    }
                    if characters[index] == '\'' {
                        index += 1;
                        break;
                    }
                    index += 1;
                }
                out.push(' ');
                continue;
            }
        }

        out.push(current);
        index += 1;
    }
    out
}

/// Whether the stripped code uses the `unsafe` keyword as a word.
fn uses_unsafe(code: &str) -> bool {
    code.split(|character: char| !(character.is_alphanumeric() || character == '_'))
        .any(|word| word == "unsafe")
}

/// The two, and only two, attribute forms that may gate an `allow(unsafe_code)` request.
///
/// Both are exact: the Windows source reader and the Linux readback seam. Anything else — a bare
/// `#[allow(unsafe_code)]`, a crate-wide `#![allow(unsafe_code)]`, an `any(..)`/`not(..)`/combined
/// or ambiguous predicate, or an ungated request — fails the match and is refused.
const APPROVED_ALLOWANCE_GATES: [&str; 2] =
    ["cfg_attr(windows,", "cfg_attr(target_os = \"linux\","];

/// Whether any `allow(unsafe_code)` is asked for **without** one of the two approved gates.
///
/// The crate root carries exactly two module-scoped allowances — the documented
/// `#[cfg_attr(windows, allow(unsafe_code))]` for the Windows source reader and the documented
/// `#[cfg_attr(target_os = "linux", allow(unsafe_code))]` for the Linux readback seam. Those two
/// exact attribute texts are removed first, on the **raw** source, because the code-only view
/// erases the string literal that carries the gate; whatever request remains after that, in code
/// rather than in a comment or a string, is an ungated or unapproved request and is refused.
fn has_ungated_allowance(source: &str) -> bool {
    let mut remainder = source.to_owned();
    for gate in APPROVED_ALLOWANCE_GATES {
        remainder = remainder.replace(&format!("#[{gate} allow(unsafe_code))]"), "");
    }
    code_only(&remainder).contains("allow(unsafe_code)")
}

/// Why `source`, located at the repository-relative `path`, is not allowed.
///
/// An empty result means compliant. This one function is what the guard tests call for both
/// the real tree and the deliberately bad fixtures.
fn violations(path: &str, source: &str) -> Vec<String> {
    let code = code_only(source);
    let mut problems = Vec::new();
    let role = role_of(path);

    // The keyword and an ungated `allow(unsafe_code)` request are both refusals: a file may not
    // use unsafe, and it may not ask for the allowance that would make using it possible without
    // a platform gate. A `deny(unsafe_code)` is the opposite of an allowance. This holds at every
    // path, including the two eligible ones, so an eligible file can never gain a blanket
    // allowance either.
    if has_ungated_allowance(source) {
        problems.push(format!(
            "{path}: asks for `allow(unsafe_code)` outside the single approved Windows gate"
        ));
    }

    if uses_unsafe(&code) && role == Role::Ordinary {
        problems.push(format!(
            "{path}: uses `unsafe` outside the two approved native boundary modules"
        ));
    }

    if uses_unsafe(&code) && role != Role::Ordinary {
        // The gate decision is the architecture checker's own AST rule. A substring that merely
        // contains the word `windows` or `linux` therefore never stands in for a positive
        // `#[cfg(..)]` proof, and a negated, alternated or malformed predicate never counts.
        if let Err(reason) = architecture_checks::source::classify_unsafe_scope(path, source) {
            problems.push(format!("{path}: {reason}"));
        }
        // Every unsafe block must be justified, so a new block cannot arrive without explaining
        // the invariants it relies on.
        let blocks = code.matches("unsafe {").count() + code.matches("unsafe{").count();
        let notes = source.matches("// SAFETY:").count();
        if blocks > notes {
            problems.push(format!(
                "{path}: {blocks} unsafe block(s) but only {notes} SAFETY note(s)"
            ));
        }
    }

    // Native roots are fixed per exception. A root that is not in the file's own list — or that
    // is a dynamic-loading root — is refused wherever it appears.
    let permitted: &[&str] = match role {
        Role::Ordinary => &[],
        Role::SourceRead => &SOURCE_READ_ROOTS,
        Role::ProcessNative => &PROCESS_NATIVE_ROOTS,
    };
    for root in ALL_NATIVE_ROOTS {
        if !code.contains(root) {
            continue;
        }
        if FORBIDDEN_ROOTS.contains(&root) || !permitted.contains(&root) {
            problems.push(format!(
                "{path}: imports a native crate `{root}` outside the roots its exception permits"
            ));
        }
    }

    problems
}

#[test]
fn no_first_party_file_outside_the_native_module_uses_unsafe() {
    let root = repository_root();
    let mut problems = Vec::new();
    for path in first_party_sources(&root) {
        let name = relative(&root, &path);
        let Ok(source) = fs::read_to_string(&path) else {
            problems.push(format!("{name}: unreadable"));
            continue;
        };
        problems.extend(violations(&name, &source));
    }
    assert!(
        problems.is_empty(),
        "unsafe or native imports escaped the approved module:\n{}",
        problems.join("\n")
    );
}

#[test]
fn the_native_module_and_its_rationales_are_present_and_justified() {
    let root = repository_root();
    let path = root.join(NATIVE_MODULE);
    let source = fs::read_to_string(&path).expect("the native module exists");
    assert!(
        violations(NATIVE_MODULE, &source).is_empty(),
        "the native module itself must justify every unsafe block"
    );
    assert!(
        code_only(&source).contains("unsafe {"),
        "the native module is expected to contain the audited unsafe blocks"
    );
}

#[test]
fn the_adapter_lint_arrangement_is_exactly_the_approved_one() {
    let root = repository_root();
    let source = fs::read_to_string(root.join(ADAPTER_ROOT)).expect("the adapter root exists");

    assert!(
        source.contains("#![deny(unsafe_code)]"),
        "the adapter must deny unsafe_code rather than forbid it"
    );
    assert!(
        !source.contains("#![forbid(unsafe_code)]"),
        "forbidding would make the one approved module impossible"
    );
    assert!(
        !source.contains("#![allow(unsafe_code)]"),
        "a crate-wide allowance is never permitted"
    );
    let windows_allowances = source
        .matches("#[cfg_attr(windows, allow(unsafe_code))]")
        .count();
    assert_eq!(
        windows_allowances, 1,
        "exactly one module-scoped, Windows-only allowance is approved"
    );
    let linux_allowances = source
        .matches("#[cfg_attr(target_os = \"linux\", allow(unsafe_code))]")
        .count();
    assert_eq!(
        linux_allowances, 1,
        "exactly one module-scoped, Linux-only allowance is approved"
    );
    assert!(
        source.contains("mod media_worker_host_native;"),
        "the Linux allowance must be scoped to the readback seam itself"
    );
    assert!(
        source.contains("mod source_read_native;"),
        "the Windows allowance must remain scoped to the source reader itself"
    );
}

#[test]
fn the_guard_refuses_an_injected_unsafe_block() {
    let injected = "fn hidden() { let value = unsafe { core::ptr::null::<u8>() }; }";
    let problems = violations("crates/solo-domain/src/planted.rs", injected);
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("uses `unsafe`")),
        "an injected unsafe block must be refused, got {problems:?}"
    );
}

#[test]
fn the_guard_refuses_an_injected_unsafe_function_and_attribute() {
    assert!(!violations("crates/solo-cli/src/x.rs", "pub unsafe fn f() {}").is_empty());
    assert!(
        !violations(
            "crates/solo-cli/src/x.rs",
            "#[allow(unsafe_code)]\nfn f() {}"
        )
        .is_empty()
    );
}

#[test]
fn the_guard_refuses_an_injected_native_import() {
    let injected = "use windows_sys::Win32::Foundation::HANDLE;";
    let problems = violations("crates/solo-application/src/planted.rs", injected);
    assert!(
        problems
            .iter()
            .any(|problem| problem.contains("native crate")),
        "an injected native import must be refused, got {problems:?}"
    );
}

#[test]
fn the_guard_refuses_a_native_block_without_a_rationale() {
    let unjustified = "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }";
    let problems = violations(NATIVE_MODULE, unjustified);
    assert!(
        problems.iter().any(|problem| problem.contains("SAFETY")),
        "an unjustified native block must be refused, got {problems:?}"
    );
}

#[test]
fn the_guard_ignores_mentions_in_comments_strings_and_lifetimes() {
    let prose = concat!(
        "// this function is unsafe to call, but it is not marked so\n",
        "/// A doc comment about unsafe code.\n",
        "/* block comment mentioning unsafe { } */\n",
        "const NOTE: &str = \"unsafe { }\";\n",
        "const RAW: &str = r#\"unsafe { }\"#;\n",
        "fn borrow<'a>(value: &'a str) -> &'a str { value }\n",
        "fn ch() -> char { '\\'' }\n",
    );
    assert!(
        violations("crates/solo-domain/src/prose.rs", prose).is_empty(),
        "a mention must never be mistaken for a use: {}",
        code_only(prose)
    );
}

// ---------------------------------------------------------------------------
// `SOLO-WO-074 R01`: the reserved second path, its OS gates and its native roots.
// ---------------------------------------------------------------------------

/// One gated `unsafe` block with an adjacent rationale, under `attribute`.
fn gated_block(attribute: &str) -> String {
    format!(
        "{attribute}\nfn f() {{\n    // SAFETY: the test only observes an address, never dereferences it.\n    let _ = unsafe {{ core::ptr::null::<u8>() }};\n}}\n"
    )
}

#[test]
fn the_reserved_process_module_now_holds_the_linux_readback_seam() {
    let root = repository_root();
    assert!(
        root.join(NATIVE_MODULE).is_file(),
        "the existing source-read module must still be present"
    );

    let source = fs::read_to_string(root.join(RESERVED_PROCESS_MODULE))
        .expect("the Linux readback seam exists after SOLO-WO-076 R01");
    assert!(
        violations(RESERVED_PROCESS_MODULE, &source).is_empty(),
        "the readback seam must be positively Linux-gated and justify every unsafe block"
    );
    assert!(
        source.contains("#[cfg(target_os = \"linux\")]"),
        "the real seam must carry its own positive Linux gate in source text"
    );
    assert!(
        code_only(&source).contains("unsafe {"),
        "the seam is expected to contain its two audited observations"
    );

    let adapter_root =
        fs::read_to_string(root.join(ADAPTER_ROOT)).expect("the adapter root exists");
    assert!(
        adapter_root.contains("mod media_worker_host_native;"),
        "the adapter root must declare the readback seam"
    );
    assert!(
        !adapter_root.contains("pub use media_worker_host_native"),
        "the seam stays private: no public host API may be exported"
    );
}

#[test]
fn the_guard_accepts_a_positively_gated_reserved_process_module() {
    for attribute in [
        "#[cfg(target_os = \"linux\")]",
        "#[cfg(windows)]",
        "#[cfg(all(test, windows))]",
        "#[cfg(all(test, target_os = \"linux\"))]",
    ] {
        let problems = violations(RESERVED_PROCESS_MODULE, &gated_block(attribute));
        assert!(
            problems.is_empty(),
            "{attribute} must be a positive gate for the reserved path: {problems:?}"
        );
    }
}

#[test]
fn the_guard_refuses_an_unproven_gate_in_the_reserved_process_module() {
    let unproven = [
        "#[cfg(any(windows, feature = \"x\"))]",
        "#[cfg(any(target_os = \"linux\", feature = \"x\"))]",
        "#[cfg(not(windows))]",
        "#[cfg(not(test))]",
        "#[cfg(target_os = \"macos\")]",
        "#[cfg_attr(windows, allow(dead_code))]",
        "",
    ];
    for attribute in unproven {
        let problems = violations(RESERVED_PROCESS_MODULE, &gated_block(attribute));
        assert!(
            !problems.is_empty(),
            "`{attribute}` must never be accepted as an OS proof"
        );
    }

    // A block without its rationale is refused even when correctly gated.
    let unjustified = "#[cfg(windows)]\nfn f() { let _ = unsafe { core::ptr::null::<u8>() }; }\n";
    assert!(
        violations(RESERVED_PROCESS_MODULE, unjustified)
            .iter()
            .any(|problem| problem.contains("SAFETY")),
        "a gated block still needs its local rationale"
    );
}

#[test]
fn the_guard_refuses_a_copied_or_aliased_native_module() {
    let gated = gated_block("#[cfg(windows)]");
    for path in [
        "crates/solo-adapters/src/media_worker_host_native_copy.rs",
        "crates/solo-adapters/src/media_worker_host_native.rs.bak",
        "crates/solo-application/src/media_worker_host_native.rs",
        "crates/solo-cli/src/media_worker_host_native.rs",
        "crates/solo-domain/src/media_worker_host_native.rs",
        "media_worker_host_native.rs",
    ] {
        assert!(
            !violations(path, &gated).is_empty(),
            "a copied module at `{path}` must be refused"
        );
    }
}

#[test]
fn the_guard_refuses_native_roots_outside_their_own_exception() {
    // `libc` is a process-control root: never in an ordinary file, and never on the source reader.
    for path in [
        "crates/solo-domain/src/lib.rs",
        "crates/solo-application/src/media_worker.rs",
        "crates/solo-adapters/src/lib.rs",
        "crates/solo-cli/src/main.rs",
        NATIVE_MODULE,
    ] {
        assert!(
            violations(path, "use libc::rlimit;\n")
                .iter()
                .any(|problem| problem.contains("native crate")),
            "`libc` must be refused at `{path}`"
        );
    }

    // `rustix` belongs to the source reader only.
    assert!(
        violations(RESERVED_PROCESS_MODULE, "use rustix::io::Errno;\n")
            .iter()
            .any(|problem| problem.contains("native crate")),
        "`rustix` is not a process-control root"
    );

    // Dynamic loading never, anywhere, in any file, under any gate.
    for path in [
        RESERVED_PROCESS_MODULE,
        NATIVE_MODULE,
        "crates/solo-domain/src/lib.rs",
    ] {
        for root in ["libloading", "dlopen", "dlsym", "libdl", "LoadLibrary"] {
            let source = format!("use {root}::Thing;\n");
            assert!(
                violations(path, &source)
                    .iter()
                    .any(|problem| problem.contains("native crate")),
                "`{root}` must be refused at `{path}`"
            );
        }
    }
}

#[test]
fn the_guard_refuses_an_allowance_that_is_not_the_single_approved_one() {
    let forms = [
        "#[allow(unsafe_code)]\nfn f() {}\n",
        "#[cfg_attr(any(target_os = \"linux\", windows), allow(unsafe_code))]\nfn f() {}\n",
        "#[cfg_attr(not(windows), allow(unsafe_code))]\nfn f() {}\n",
        "#![allow(unsafe_code)]\nfn f() {}\n",
    ];
    for source in forms {
        for path in [ADAPTER_ROOT, RESERVED_PROCESS_MODULE, NATIVE_MODULE] {
            assert!(
                !violations(path, source).is_empty(),
                "an unapproved allowance form must be refused at `{path}`"
            );
        }
    }

    // Both approved gated forms stay legal at the adapter root.
    assert!(
        violations(
            ADAPTER_ROOT,
            "#[cfg_attr(windows, allow(unsafe_code))]\nmod native;\n"
        )
        .is_empty(),
        "the single documented Windows-gated allowance must remain legal"
    );
    assert!(
        violations(
            ADAPTER_ROOT,
            "#[cfg_attr(target_os = \"linux\", allow(unsafe_code))]\nmod native;\n"
        )
        .is_empty(),
        "the single documented Linux-gated allowance must be legal too"
    );

    // A combined predicate is neither of the two approved forms, at any path.
    assert!(
        !violations(
            ADAPTER_ROOT,
            "#[cfg_attr(any(target_os = \"linux\", windows), allow(unsafe_code))]\nmod native;\n"
        )
        .is_empty(),
        "a combined `any(..)` gate is never an approved allowance"
    );
}
