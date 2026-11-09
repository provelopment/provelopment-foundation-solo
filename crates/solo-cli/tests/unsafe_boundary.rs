//! Load-bearing guard: `unsafe` is confined to one Windows-gated module.
//!
//! The architecture allows exactly one project-owned `unsafe` region, because the Windows
//! source reader must call `NtCreateFile` directly. Everywhere else the code is safe Rust. A
//! comment in a crate header cannot enforce that, so this file does it mechanically:
//!
//! * every first-party `.rs` file is scanned after comments and string literals are removed,
//!   so a *mention* of the word never counts as a use of it;
//! * only `crates/solo-adapters/src/source_read_native.rs` may use the keyword or import the
//!   native crates;
//! * every `unsafe` block in that one file must carry an adjacent `// SAFETY:` rationale;
//! * the adapter's lint arrangement must be exactly `#![deny(unsafe_code)]` plus a single
//!   `#[cfg_attr(windows, allow(unsafe_code))]` on the native module.
//!
//! The final tests feed deliberately bad sources through the same functions, so the guard is
//! proven to refuse rather than merely to pass.

use std::fs;
use std::path::{Path, PathBuf};

#[path = "architecture_checks/mod.rs"]
mod architecture_checks;

use architecture_checks::source::{APPROVED_NATIVE_MODULE, classify_unsafe_scope};

/// The single file permitted to contain `unsafe` and to import the native crates.
const NATIVE_MODULE: &str = "crates/solo-adapters/src/source_read_native.rs";

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

/// Why `source`, located at the repository-relative `path`, is not allowed.
///
/// An empty result means compliant. This one function is what the guard tests call for both
/// the real tree and the deliberately bad fixtures.
fn violations(path: &str, source: &str) -> Vec<String> {
    let code = code_only(source);
    let mut problems = Vec::new();
    let native = path == NATIVE_MODULE;

    if uses_unsafe(&code) && !native {
        problems.push(format!("{path}: uses `unsafe` outside the approved module"));
    }
    if (code.contains("windows_sys") || code.contains("rustix")) && !native {
        problems.push(format!(
            "{path}: imports a native crate outside the approved module"
        ));
    }
    if native {
        // Every unsafe block must be justified, so a new block cannot arrive without
        // explaining the invariants it relies on.
        let blocks = code.matches("unsafe {").count() + code.matches("unsafe{").count();
        let notes = source.matches("// SAFETY:").count();
        if blocks > notes {
            problems.push(format!(
                "{path}: {blocks} unsafe block(s) but only {notes} SAFETY note(s)"
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
    let allowances = source
        .matches("#[cfg_attr(windows, allow(unsafe_code))]")
        .count();
    assert_eq!(
        allowances, 1,
        "exactly one module-scoped, Windows-only allowance is approved"
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

/// The architecture checker's own rule must permit exactly what this guard permits.
///
/// These cases are the negative proof for the Owner-approved exception: the rule needs both
/// the exact approved module and a real `#[cfg(windows)]` scope, and refuses every
/// near-miss that would otherwise be a hole.
#[test]
fn the_checker_allows_unsafe_only_inside_the_windows_gated_approved_module() {
    let gated = concat!(
        "#[cfg(windows)]\n",
        "mod sys {\n",
        "    fn f() {\n",
        "        let _ = unsafe { core::ptr::null::<u8>() };\n",
        "    }\n",
        "}\n",
    );
    assert!(
        classify_unsafe_scope(APPROVED_NATIVE_MODULE, gated).is_ok(),
        "the approved module may use unsafe inside a Windows gate"
    );
    assert!(
        classify_unsafe_scope("crates/solo-domain/src/lib.rs", gated).is_err(),
        "the same code outside the approved module must be refused"
    );
    assert!(
        classify_unsafe_scope("crates/solo-adapters/src/source_read.rs", gated).is_err(),
        "the safe wrapper must never be allowed an unsafe block"
    );
}

#[test]
fn the_checker_refuses_unsafe_outside_a_positive_windows_gate() {
    let unscoped = "fn f() { let _ = unsafe { core::ptr::null::<u8>() }; }\n";
    let reason = classify_unsafe_scope(APPROVED_NATIVE_MODULE, unscoped)
        .expect_err("unscoped unsafe must be refused even in the approved module");
    assert!(
        reason.contains("outside a `#[cfg(windows)]` scope"),
        "{reason}"
    );

    let negated = concat!(
        "#[cfg(not(windows))]\n",
        "mod sys { fn f() { let _ = unsafe { core::ptr::null::<u8>() }; } }\n",
    );
    assert!(
        classify_unsafe_scope(APPROVED_NATIVE_MODULE, negated).is_err(),
        "a negated gate is not a Windows gate"
    );

    let other_os = concat!(
        "#[cfg(target_os = \"linux\")]\n",
        "mod sys { fn f() { let _ = unsafe { core::ptr::null::<u8>() }; } }\n",
    );
    assert!(
        classify_unsafe_scope(APPROVED_NATIVE_MODULE, other_os).is_err(),
        "a non-Windows gate is not a Windows gate"
    );

    assert!(
        classify_unsafe_scope(APPROVED_NATIVE_MODULE, "pub unsafe fn f() {}").is_err(),
        "an ungated unsafe fn must be refused even in the approved module"
    );
}

#[test]
fn the_approved_module_passes_the_checker_rule_and_no_other_file_does() {
    let root = repository_root();

    let native = fs::read_to_string(root.join(APPROVED_NATIVE_MODULE))
        .expect("the approved native module exists");
    assert!(
        classify_unsafe_scope(APPROVED_NATIVE_MODULE, &native).is_ok(),
        "the real native module must satisfy the checker's rule"
    );

    for path in first_party_sources(&root) {
        let name = relative(&root, &path);
        if name == APPROVED_NATIVE_MODULE {
            continue;
        }
        let source = fs::read_to_string(&path).expect("first-party source is readable");
        assert!(
            classify_unsafe_scope(&name, &source).is_ok(),
            "{name} must contain no unsafe the checker would refuse"
        );
    }
}
