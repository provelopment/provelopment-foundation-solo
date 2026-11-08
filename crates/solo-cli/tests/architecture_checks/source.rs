//! Fail-closed `syn` AST source-boundary checker (Architecture §6A.3).
//!
//! What it inspects: every Rust file reachable from a declared Cargo target, following
//! out-of-line and inline modules, `#[path]`, literal `include!` and the arguments of an
//! explicitly audited macro allowlist. What it refuses rather than guesses: locally
//! defined or unknown macros, generated/exotic items (`syn::Item::Verbatim`), `extern`
//! blocks, dynamic loading, escapes outside the package root and any path whose root it
//! cannot resolve.
//!
//! It is honest about its limit: this is token/AST analysis, **not** compiler macro
//! expansion or name resolution. Anything it cannot inspect is forbidden, never
//! exempted.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Component, Path, PathBuf};

use syn::visit::Visit;

use super::graph::{self, DepKind, PackageTargets, TargetKind};
use super::{Category, Violation};

/// Why a candidate source path could not be proven confined.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfinementError {
    /// The lexical or the effective path lies outside the package root.
    Escape,
    /// The path could not be inspected or canonicalised, so it cannot be proven safe.
    Unresolvable,
}

/// Reads source text. The real tree uses [`FsReader`]; fixtures use an in-memory map,
/// so a negative case never has to touch the working tree.
///
/// [`SourceReader::confine`] is the single confinement gate: every source the checker may
/// open, parse or treat as reachable is proved safe through it first. It fails closed —
/// an unprovable path is an error, never a silent skip.
pub trait SourceReader {
    fn read(&self, path: &Path) -> Option<String>;
    fn is_file(&self, path: &Path) -> bool;
    fn confine(&self, path: &Path, root: &Path, anchor: &Path)
    -> Result<PathBuf, ConfinementError>;
}

/// Whether a filesystem entry redirects elsewhere: a symlink, a Windows junction, or any
/// other reparse point.
fn is_redirect(metadata: &std::fs::Metadata) -> bool {
    if metadata.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        false
    }
}

/// Proves that no original filesystem entry from the volume root down to `target` is a
/// redirect, and that every intermediate entry is a real directory.
///
/// This inspects the **unresolved** entries, so a redirected package root — even one whose
/// target is another directory inside the same checkout — is refused before its contents
/// are opened. Canonicalising a redirected root and its redirected child consistently is
/// exactly the false positive this check exists to prevent.
pub fn inspect_original_chain(target: &Path) -> Result<(), ConfinementError> {
    let components: Vec<Component> = target.components().collect();
    let mut current = PathBuf::new();

    for (index, component) in components.iter().enumerate() {
        current.push(component.as_os_str());
        if matches!(
            component,
            Component::Prefix(_) | Component::RootDir | Component::CurDir
        ) {
            continue;
        }

        let metadata =
            std::fs::symlink_metadata(&current).map_err(|_| ConfinementError::Unresolvable)?;
        if is_redirect(&metadata) {
            return Err(ConfinementError::Escape);
        }

        let last = index + 1 == components.len();
        if last {
            if !(metadata.is_dir() || metadata.is_file()) {
                return Err(ConfinementError::Unresolvable);
            }
        } else if !metadata.is_dir() {
            return Err(ConfinementError::Escape);
        }
    }

    Ok(())
}

/// The verified checkout anchor every package root is anchored to.
///
/// It is derived from locked Cargo metadata plus the real Solo tree, never from a
/// package's self-declared root, and it is validated before any source is used.
pub struct CheckoutAnchor {
    root: PathBuf,
}

impl CheckoutAnchor {
    pub fn verify(
        workspace_root: &Path,
        packages: &[PackageTargets],
    ) -> Result<Self, Vec<Violation>> {
        let root = normalize_lexical(workspace_root);

        match inspect_original_chain(&root) {
            Ok(()) => {}
            Err(ConfinementError::Escape) => {
                return Err(vec![Violation::new(
                    Category::ModuleSourceEscape,
                    "workspace: checkout anchor",
                    "the checkout anchor is reached through a link or reparse point",
                )]);
            }
            Err(ConfinementError::Unresolvable) => {
                return Err(vec![Violation::new(
                    Category::UnreadableSource,
                    "workspace: checkout anchor",
                    "the checkout anchor could not be inspected",
                )]);
            }
        }

        let manifest = root.join("Cargo.toml");
        let manifest_ok = std::fs::symlink_metadata(&manifest)
            .is_ok_and(|metadata| metadata.is_file() && !is_redirect(&metadata));
        let workspace_manifest = manifest_ok
            && std::fs::read_to_string(&manifest).is_ok_and(|text| text.contains("[workspace]"));
        if !workspace_manifest {
            return Err(vec![Violation::new(
                Category::UnreadableSource,
                "workspace: checkout anchor",
                "the checkout anchor has no inspectable workspace manifest",
            )]);
        }

        if std::fs::symlink_metadata(root.join(".git")).is_err() {
            return Err(vec![Violation::new(
                Category::UnreadableSource,
                "workspace: checkout anchor",
                "the checkout anchor is not a Git working tree root",
            )]);
        }

        let mut violations: Vec<Violation> = Vec::new();
        for package in packages {
            let expected = root.join("crates").join(&package.name);
            if normalize_lexical(&package.root) != normalize_lexical(&expected) {
                violations.push(Violation::new(
                    Category::ModuleSourceEscape,
                    format!("crates/{}", package.name),
                    "package root is not its declared crates/<package> path under the anchor",
                ));
            }
        }

        if !violations.is_empty() {
            return Err(violations);
        }

        Ok(Self { root })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

/// Read-only filesystem access with real effective-path enforcement.
pub struct FsReader;

impl SourceReader for FsReader {
    fn read(&self, path: &Path) -> Option<String> {
        std::fs::read_to_string(path).ok()
    }

    fn is_file(&self, path: &Path) -> bool {
        std::fs::metadata(path).is_ok_and(|metadata| metadata.is_file())
    }

    /// Lexical containment, then the **original** entry chain from the volume root through
    /// the checkout anchor and the package root down to the file, then canonicalisation of
    /// both the file and the package root. Any failure refuses.
    ///
    /// This cannot eliminate a hostile time-of-check/time-of-use race; it proves the entry
    /// that is about to be read is owned at the moment it is checked.
    fn confine(
        &self,
        path: &Path,
        root: &Path,
        anchor: &Path,
    ) -> Result<PathBuf, ConfinementError> {
        let lexical = normalize_lexical(path);
        let lexical_root = normalize_lexical(root);
        let lexical_anchor = normalize_lexical(anchor);

        if !is_inside(&lexical, &lexical_root) || !is_inside(&lexical_root, &lexical_anchor) {
            return Err(ConfinementError::Escape);
        }

        inspect_original_chain(&lexical_anchor)?;
        inspect_original_chain(&lexical_root)?;
        inspect_original_chain(&lexical)?;

        let effective =
            std::fs::canonicalize(&lexical).map_err(|_| ConfinementError::Unresolvable)?;
        let effective_root =
            std::fs::canonicalize(&lexical_root).map_err(|_| ConfinementError::Unresolvable)?;
        if !is_inside(
            &normalize_lexical(&effective),
            &normalize_lexical(&effective_root),
        ) {
            return Err(ConfinementError::Escape);
        }

        Ok(effective)
    }
}

/// In-memory reader for the negative fixtures.
#[derive(Default)]
pub struct MapReader {
    files: BTreeMap<PathBuf, String>,
}

impl MapReader {
    pub fn with(mut self, path: impl Into<PathBuf>, text: impl Into<String>) -> Self {
        self.files.insert(path.into(), text.into());
        self
    }

    pub fn new() -> Self {
        Self::default()
    }
}

impl SourceReader for MapReader {
    fn read(&self, path: &Path) -> Option<String> {
        self.files.get(path).cloned()
    }

    fn is_file(&self, path: &Path) -> bool {
        self.files.contains_key(path)
    }

    /// In-memory fixtures never consult the host filesystem: confinement is the lexical
    /// package-root rule only, so a fixture can express an escape intent without needing
    /// a real link.
    fn confine(
        &self,
        path: &Path,
        root: &Path,
        anchor: &Path,
    ) -> Result<PathBuf, ConfinementError> {
        let lexical = normalize_lexical(path);
        if !is_inside(&lexical, &normalize_lexical(root))
            || !is_inside(&normalize_lexical(root), &normalize_lexical(anchor))
        {
            return Err(ConfinementError::Escape);
        }
        Ok(lexical)
    }
}

/// The public root surface of a crate: the names another crate may legally name after
/// its crate root. Anything else is a private internal.
#[derive(Debug, Clone, Default)]
pub struct PublicSurface {
    pub names: BTreeSet<String>,
}

impl PublicSurface {
    pub fn of(names: &[&str]) -> Self {
        Self {
            names: names.iter().map(|name| (*name).to_owned()).collect(),
        }
    }
}

/// One crate root to scan, with the policy that applies to it.
#[derive(Debug, Clone)]
pub struct UnitSpec {
    /// Workspace package owning the root, e.g. `solo-application`.
    pub package: String,
    pub kind: TargetKind,
    /// Crate root file (absolute for the real tree).
    pub root_file: PathBuf,
    /// Directory boundary: no resolved source may escape it.
    pub package_root: PathBuf,
    /// Verified checkout anchor this package root must sit under (`crates/<package>`).
    pub anchor: PathBuf,
    /// Declared workspace dependency roots: extern ident -> package name.
    pub workspace_roots: BTreeMap<String, String>,
    /// External crates usable in this context (normal deps plus dev deps in tests).
    pub allowed_external: BTreeSet<String>,
    /// Externals declared only as dev-dependencies of this package.
    pub dev_roots: BTreeSet<String>,
    /// Every workspace crate root ident, whether declared here or not.
    pub known_crates: BTreeSet<String>,
    /// Public surfaces of the workspace crates, keyed by package name.
    pub surfaces: BTreeMap<String, PublicSurface>,
    /// Diagnostic label used instead of an absolute path.
    pub label_root: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SourceReport {
    pub units: usize,
    pub files: usize,
}

/// Macro names audited as harmless in this source tree. Their arguments are still
/// parsed and every path inside them is classified; the list is about *expansion*
/// safety, not about skipping analysis.
pub const AUDITED_MACROS: [&str; 21] = [
    "assert",
    "assert_eq",
    "assert_ne",
    "concat",
    "debug_assert",
    "debug_assert_eq",
    "debug_assert_ne",
    "eprint",
    "eprintln",
    "env",
    "format",
    "format_args",
    "matches",
    "option_env",
    "panic",
    "print",
    "println",
    "stringify",
    "vec",
    "write",
    "writeln",
];

/// Macros that load source text and are therefore never transparent.
const SOURCE_LOADING_MACROS: [&str; 3] = ["include", "include_bytes", "include_str"];

/// Roots that would let checked source reach a dynamically loaded object.
const DYNAMIC_LOADING_ROOTS: [&str; 6] =
    ["dlopen", "libc", "libloading", "libdl", "region", "memmap2"];

const PRIMITIVE_ROOTS: [&str; 17] = [
    "bool", "char", "f32", "f64", "i128", "i16", "i32", "i64", "i8", "isize", "str", "u128", "u16",
    "u32", "u64", "u8", "usize",
];

#[derive(Clone)]
struct FileCtx {
    path: PathBuf,
    dir: PathBuf,
    /// True when children of this file live directly in `dir` (`lib.rs`, `main.rs`,
    /// `mod.rs` and every integration-test root behave this way).
    is_crate_root: bool,
    stem: String,
    test_context: bool,
    /// Imported name -> full used path, propagated into nested modules.
    aliases: BTreeMap<String, Vec<String>>,
}

struct Scanner<'a> {
    reader: &'a dyn SourceReader,
    spec: &'a UnitSpec,
    violations: Vec<Violation>,
    visited: BTreeSet<PathBuf>,
    /// Module names declared in this crate, learned by a structural pre-pass.
    local_modules: BTreeSet<String>,
    /// When set, the scan only records module structure (used by the pre-pass).
    collect_only: bool,
}

/// Scans one crate root and returns every refusal it found.
pub fn check_unit(spec: &UnitSpec, reader: &dyn SourceReader) -> Vec<Violation> {
    let root = normalize_lexical(&spec.root_file);

    let mut pre_pass = Scanner {
        reader,
        spec,
        violations: Vec::new(),
        visited: BTreeSet::new(),
        local_modules: BTreeSet::new(),
        collect_only: true,
    };
    if !is_inside(&root, &normalize_lexical(&spec.package_root)) {
        return vec![Violation::new(
            Category::ModuleSourceEscape,
            format!("{}/<outside>", spec.label_root),
            "crate root lies outside its package root",
        )];
    }
    pre_pass.scan(FileCtx {
        dir: root.parent().map(Path::to_path_buf).unwrap_or_default(),
        stem: file_stem(&root),
        is_crate_root: true,
        test_context: spec.kind.is_test_context(),
        aliases: BTreeMap::new(),
        path: root.clone(),
    });
    let local_modules = pre_pass.local_modules;

    let mut scanner = Scanner {
        reader,
        spec,
        violations: Vec::new(),
        visited: BTreeSet::new(),
        local_modules,
        collect_only: false,
    };
    scanner.scan(FileCtx {
        dir: root.parent().map(Path::to_path_buf).unwrap_or_default(),
        stem: file_stem(&root),
        is_crate_root: true,
        test_context: spec.kind.is_test_context(),
        aliases: BTreeMap::new(),
        path: root,
    });
    sort_unique_local(scanner.violations)
}

fn sort_unique_local(violations: Vec<Violation>) -> Vec<Violation> {
    let set: BTreeSet<Violation> = violations.into_iter().collect();
    set.into_iter().collect()
}

/// Scans every declared target of every workspace package.
pub fn check_packages(
    packages: &[PackageTargets],
    workspace_root: &Path,
) -> Result<SourceReport, Vec<Violation>> {
    let reader = FsReader;
    let anchor = CheckoutAnchor::verify(workspace_root, packages)?;

    let (surfaces, mut violations) = public_surfaces(packages, &reader, anchor.root());
    let known_crates: BTreeSet<String> = packages
        .iter()
        .map(|package| package.name.replace('-', "_"))
        .collect();

    let mut files: BTreeSet<PathBuf> = BTreeSet::new();
    let mut units = 0usize;

    for package in packages {
        let mut workspace_roots: BTreeMap<String, String> = BTreeMap::new();
        let mut dev_roots: BTreeSet<String> = BTreeSet::new();
        let mut normal_roots: BTreeSet<String> = BTreeSet::new();
        if let Some(lib) = &package.lib_crate {
            workspace_roots.insert(lib.clone(), package.name.clone());
        }
        for dep in &package.deps {
            if graph::is_workspace(&dep.package) {
                workspace_roots.insert(dep.root_ident.clone(), dep.package.clone());
            } else if dep.kind == DepKind::Dev {
                dev_roots.insert(dep.root_ident.clone());
            } else {
                normal_roots.insert(dep.root_ident.clone());
            }
        }

        for target in &package.targets {
            if target.kind == TargetKind::CustomBuild {
                violations.push(Violation::new(
                    Category::UnsupportedTarget,
                    format!("{}/{}", label_root(&package.root), target.name),
                    "build scripts are not inspectable by this checker and are refused",
                ));
                continue;
            }

            let mut allowed_external = normal_roots.clone();
            if target.kind.is_test_context() {
                allowed_external.extend(dev_roots.iter().cloned());
            }
            let spec = UnitSpec {
                package: package.name.clone(),
                kind: target.kind,
                root_file: target.src_path.clone(),
                package_root: package.root.clone(),
                anchor: anchor.root().to_path_buf(),
                workspace_roots: workspace_roots.clone(),
                allowed_external,
                dev_roots: dev_roots.clone(),
                known_crates: known_crates.clone(),
                surfaces: surfaces.clone(),
                label_root: label_root(&package.root),
            };
            units += 1;
            let found = check_unit(&spec, &reader);
            files.extend(visited_files(&spec, &reader));
            violations.extend(found);
        }
    }

    // Hidden source: every Rust file in a package must be reachable from a declared
    // target, otherwise an unchecked file could carry a forbidden edge.
    for package in packages {
        let label = label_root(&package.root);
        let (discovered, mut walk_violations) =
            rust_files_under(&package.root, &label, anchor.root(), &RealDiscoveryFs);
        violations.append(&mut walk_violations);
        for path in discovered {
            if files.contains(&path) {
                continue;
            }
            violations.push(Violation::new(
                Category::HiddenSource,
                format!(
                    "{label}/{}",
                    path.strip_prefix(&package.root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/")
                ),
                "Rust source is not reachable from any declared Cargo target",
            ));
        }
    }

    if violations.is_empty() {
        Ok(SourceReport {
            units,
            files: files.len(),
        })
    } else {
        Err(sort_unique_local(violations))
    }
}

fn visited_files(spec: &UnitSpec, reader: &dyn SourceReader) -> BTreeSet<PathBuf> {
    let mut scanner = Scanner {
        reader,
        spec,
        violations: Vec::new(),
        visited: BTreeSet::new(),
        local_modules: BTreeSet::new(),
        collect_only: false,
    };
    let root = normalize_lexical(&spec.root_file);
    if is_inside(&root, &normalize_lexical(&spec.package_root)) {
        scanner.scan(FileCtx {
            dir: root.parent().map(Path::to_path_buf).unwrap_or_default(),
            stem: file_stem(&root),
            is_crate_root: true,
            test_context: spec.kind.is_test_context(),
            aliases: BTreeMap::new(),
            path: root,
        });
    }
    scanner.visited
}

impl Scanner<'_> {
    fn rel(&self, path: &Path) -> String {
        let root = normalize_lexical(&self.spec.package_root);
        match path.strip_prefix(&root) {
            Ok(tail) => format!(
                "{}/{}",
                self.spec.label_root,
                tail.to_string_lossy().replace('\\', "/")
            ),
            Err(_) => format!("{}/<outside>", self.spec.label_root),
        }
    }

    fn violation(&mut self, category: Category, location: String, detail: impl Into<String>) {
        self.violations
            .push(Violation::new(category, location, detail));
    }

    fn scan(&mut self, ctx: FileCtx) {
        if !self.visited.insert(ctx.path.clone()) {
            return;
        }
        let location = self.rel(&ctx.path);

        // Nothing is opened before its effective destination is proven confined.
        match self
            .reader
            .confine(&ctx.path, &self.spec.package_root, &self.spec.anchor)
        {
            Ok(_effective) => {}
            Err(ConfinementError::Escape) => {
                self.violation(
                    Category::ModuleSourceEscape,
                    location,
                    "source file is redirected outside its package root and is refused",
                );
                return;
            }
            Err(ConfinementError::Unresolvable) => {
                self.violation(
                    Category::UnreadableSource,
                    location,
                    "source file could not be proven confined (inspect or canonicalise failed)",
                );
                return;
            }
        }

        let Some(text) = self.reader.read(&ctx.path) else {
            self.violation(
                Category::UnreadableSource,
                location,
                "required source file could not be read",
            );
            return;
        };

        let file = match syn::parse_file(&text) {
            Ok(file) => file,
            Err(_) => {
                self.violation(
                    Category::UnparsableSource,
                    location,
                    "source file is not parsable Rust and cannot be inspected",
                );
                return;
            }
        };

        let mut ctx = ctx;
        self.scan_items(&file.items, &mut ctx);
    }

    fn scan_items(&mut self, items: &[syn::Item], ctx: &mut FileCtx) {
        for item in items {
            match item {
                syn::Item::Mod(module) => self.scan_module(module, ctx),
                other => self.scan_item(other, ctx),
            }
        }
    }

    fn scan_module(&mut self, module: &syn::ItemMod, ctx: &mut FileCtx) {
        self.local_modules.insert(module.ident.to_string());

        let mut facts = Facts::default();
        for attribute in &module.attrs {
            facts.visit_attribute(attribute);
        }

        let mut child = FileCtx {
            path: ctx.path.clone(),
            dir: ctx.dir.clone(),
            is_crate_root: ctx.is_crate_root,
            stem: ctx.stem.clone(),
            test_context: ctx.test_context || cfg_is_test(&module.attrs),
            aliases: ctx.aliases.clone(),
        };
        child.aliases.extend(facts.aliases.clone());
        self.report_facts(&facts, &child);

        if let Some((_, inner)) = &module.content {
            // `#[path]` on an inline module is meaningless: refuse rather than ignore it.
            if !matches!(
                module_path_selection(&module.attrs),
                ModulePathSelection::Default
            ) {
                let location = self.rel(&child.path);
                self.violation(
                    Category::ConditionalModulePath,
                    location,
                    "inline module carries a path-selecting attribute that is refused",
                );
            }
            self.scan_items(inner, &mut child);
            return;
        }

        let name = module.ident.to_string();
        let resolved = match module_path_selection(&module.attrs) {
            ModulePathSelection::Default => self.resolve_default_module(&child, &name),
            ModulePathSelection::Literal(literal) => {
                self.resolve_declared_path(&child, &literal, &name)
            }
            ModulePathSelection::Refused(category, detail) => {
                let location = self.rel(&child.path);
                self.violation(category, location, detail);
                None
            }
        };
        let Some(resolved) = resolved else {
            return;
        };

        let is_mod_file = file_stem(&resolved) == "mod";
        let next = FileCtx {
            dir: resolved.parent().map(Path::to_path_buf).unwrap_or_default(),
            stem: if is_mod_file {
                resolved
                    .parent()
                    .and_then(Path::file_name)
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_default()
            } else {
                file_stem(&resolved)
            },
            is_crate_root: is_mod_file,
            test_context: child.test_context,
            aliases: child.aliases.clone(),
            path: resolved,
        };
        self.scan(next);
    }

    /// `#[path = "..."]`: only one literal, confined, existing file is accepted.
    fn resolve_declared_path(
        &mut self,
        ctx: &FileCtx,
        literal: &str,
        name: &str,
    ) -> Option<PathBuf> {
        if literal.trim().is_empty() || literal.contains('\0') {
            let location = self.rel(&ctx.path);
            self.violation(
                Category::ModuleSourceEscape,
                location,
                format!("module `{name}` declares an empty or invalid #[path]"),
            );
            return None;
        }

        let candidate = normalize_lexical(&ctx.dir.join(literal));
        self.confined_source(
            &candidate,
            &ctx.path,
            Category::ModuleSourceEscape,
            Category::ModuleSourceUnresolved,
            &format!("module `{name}` #[path]"),
        )
    }

    /// Default `mod name;` resolution: exactly one of `name.rs` / `name/mod.rs` may
    /// exist, next to this file (`lib.rs`/`main.rs`/`mod.rs`/test roots) or in the
    /// sibling directory otherwise.
    fn resolve_default_module(&mut self, ctx: &FileCtx, name: &str) -> Option<PathBuf> {
        let base = if ctx.is_crate_root {
            ctx.dir.clone()
        } else {
            ctx.dir.join(&ctx.stem)
        };

        let candidates = [
            normalize_lexical(&base.join(format!("{name}.rs"))),
            normalize_lexical(&base.join(name).join("mod.rs")),
        ];
        let existing: Vec<PathBuf> = candidates
            .into_iter()
            .filter(|candidate| self.reader.is_file(candidate))
            .collect();

        match existing.len() {
            0 => {
                let location = self.rel(&ctx.path);
                self.violation(
                    Category::ModuleSourceUnresolved,
                    location,
                    format!("module `{name}` has no resolvable source file"),
                );
                None
            }
            1 => self.confined_source(
                &existing[0],
                &ctx.path,
                Category::ModuleSourceEscape,
                Category::ModuleSourceUnresolved,
                &format!("module `{name}`"),
            ),
            _ => {
                let location = self.rel(&ctx.path);
                self.violation(
                    Category::ModuleSourceAmbiguous,
                    location,
                    format!("module `{name}` resolves to two conflicting source files"),
                );
                None
            }
        }
    }

    /// Proves a candidate source is confined, present and identifiable **before** it is
    /// read, parsed, traversed or treated as reachable. Any unprovable path refuses; there
    /// is no warning-and-continue path.
    fn confined_source(
        &mut self,
        candidate: &Path,
        at: &Path,
        escape_category: Category,
        missing_category: Category,
        purpose: &str,
    ) -> Option<PathBuf> {
        let location = self.rel(at);
        let lexical = normalize_lexical(candidate);

        match self
            .reader
            .confine(&lexical, &self.spec.package_root, &self.spec.anchor)
        {
            Ok(_effective) => {}
            Err(ConfinementError::Escape) => {
                self.violation(
                    escape_category,
                    location,
                    format!("{purpose} leaves the package root (lexically or through a redirect)"),
                );
                return None;
            }
            Err(ConfinementError::Unresolvable) => {
                self.violation(
                    Category::UnreadableSource,
                    location,
                    format!(
                        "{purpose} could not be proven confined (inspect or canonicalise failed)"
                    ),
                );
                return None;
            }
        }

        if !self.reader.is_file(&lexical) {
            self.violation(
                missing_category,
                location,
                format!("{purpose} does not exist"),
            );
            return None;
        }

        Some(lexical)
    }

    fn scan_item(&mut self, item: &syn::Item, ctx: &mut FileCtx) {
        let mut facts = Facts::default();
        facts.visit_item(item);
        ctx.aliases.extend(facts.aliases.clone());
        self.report_facts(&facts, ctx);
    }

    fn report_facts(&mut self, facts: &Facts, ctx: &FileCtx) {
        if self.collect_only {
            return;
        }

        let location = self.rel(&ctx.path);

        if facts.unsafe_found {
            self.violation(
                Category::UnsafeCode,
                location.clone(),
                "unsafe code is forbidden in this workspace",
            );
        }
        if facts.foreign_found {
            self.violation(
                Category::ForeignBlock,
                location.clone(),
                "extern/FFI blocks are not inspectable and are refused",
            );
        }
        if facts.verbatim_found {
            self.violation(
                Category::UnsupportedItem,
                location.clone(),
                "unrecognised (verbatim) syntax cannot be inspected and is refused",
            );
        }

        for path in &facts.paths {
            self.classify(path, ctx);
        }
        for segments in &facts.use_paths {
            self.classify_segments(segments, ctx);
        }

        let mut pending: VecDeque<syn::Macro> = facts.macros.iter().cloned().collect();
        let mut budget = 512usize;
        while let Some(invocation) = pending.pop_front() {
            if budget == 0 {
                self.violation(
                    Category::MacroUnsupported,
                    location.clone(),
                    "macro analysis budget exhausted; expansion is refused",
                );
                break;
            }
            budget -= 1;

            let name = invocation
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string())
                .unwrap_or_default();
            let qualified = invocation.path.segments.len() > 1;

            if name == "include" {
                self.handle_include(&invocation, ctx);
                continue;
            }

            if SOURCE_LOADING_MACROS.contains(&name.as_str()) {
                self.violation(
                    Category::IncludeUnsupported,
                    location.clone(),
                    format!("source-loading macro `{name}` is refused"),
                );
                continue;
            }

            if qualified || !AUDITED_MACROS.contains(&name.as_str()) {
                self.violation(
                    Category::MacroUnsupported,
                    location.clone(),
                    format!("macro `{name}` cannot be soundly inspected and is refused"),
                );
                continue;
            }

            self.scan_macro_arguments(&invocation, ctx, &mut pending);
        }
    }

    /// Audited macro arguments are parsed back into real AST nodes, so a path hidden
    /// inside `format!`/`matches!`/`assert!` is classified exactly like one written
    /// outside a macro. Arguments that do not parse are refused.
    fn scan_macro_arguments(
        &mut self,
        invocation: &syn::Macro,
        ctx: &FileCtx,
        pending: &mut VecDeque<syn::Macro>,
    ) {
        use syn::parse::Parser;

        let location = self.rel(&ctx.path);
        let mut facts = Facts::default();

        let parsed_file = syn::parse2::<syn::File>(invocation.tokens.clone()).ok();
        if let Some(file) = parsed_file {
            facts.visit_file(&file);
        } else {
            let exprs =
                syn::punctuated::Punctuated::<syn::Expr, syn::token::Comma>::parse_terminated
                    .parse2(invocation.tokens.clone())
                    .ok();
            let pats = (|input: syn::parse::ParseStream<'_>| {
                syn::punctuated::Punctuated::<syn::Pat, syn::token::Comma>::parse_terminated_with(
                    input,
                    syn::Pat::parse_multi,
                )
            })
            .parse2(invocation.tokens.clone())
            .ok();
            let stmts = (|input: syn::parse::ParseStream<'_>| syn::Block::parse_within(input))
                .parse2(invocation.tokens.clone())
                .ok();

            if let Some(exprs) = exprs {
                for expression in &exprs {
                    facts.visit_expr(expression);
                }
            } else if let Some(pats) = pats {
                for pattern in &pats {
                    facts.visit_pat(pattern);
                }
            } else if let Some(stmts) = stmts {
                for statement in &stmts {
                    facts.visit_stmt(statement);
                }
            } else {
                self.violation(
                    Category::MacroUnsupported,
                    location,
                    "macro arguments are not parseable and cannot be inspected",
                );
                return;
            }
        }

        for path in &facts.paths {
            self.classify(path, ctx);
        }
        for segments in &facts.use_paths {
            self.classify_segments(segments, ctx);
        }
        pending.extend(facts.macros);
    }

    /// `include!("literal")`: the literal is resolved inside the package root and its
    /// content is parsed and inspected rather than skipped.
    fn handle_include(&mut self, invocation: &syn::Macro, ctx: &FileCtx) {
        let location = self.rel(&ctx.path);
        let Ok(literal) = syn::parse2::<syn::LitStr>(invocation.tokens.clone()) else {
            self.violation(
                Category::IncludeUnsupported,
                location,
                "include! with a non-literal path cannot be inspected and is refused",
            );
            return;
        };

        let candidate = normalize_lexical(&ctx.dir.join(literal.value()));
        let Some(confined) = self.confined_source(
            &candidate,
            &ctx.path,
            Category::IncludeEscape,
            Category::IncludeUnsupported,
            "include!",
        ) else {
            return;
        };

        let Some(text) = self.reader.read(&confined) else {
            let location = self.rel(&ctx.path);
            self.violation(
                Category::IncludeUnsupported,
                location,
                "include! target could not be read and is refused",
            );
            return;
        };

        let Ok(file) = syn::parse_file(&text) else {
            let location = self.rel(&confined);
            self.violation(
                Category::UnparsableSource,
                location,
                "included source is not parsable Rust",
            );
            return;
        };

        let mut inner = ctx.clone();
        inner.dir = confined.parent().map(Path::to_path_buf).unwrap_or_default();
        inner.path = confined;
        self.scan_items(&file.items, &mut inner);
    }

    fn classify(&mut self, path: &syn::Path, ctx: &FileCtx) {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        self.classify_segments(&segments, ctx);
    }

    /// Classifies one path, after substituting every `use`/`extern crate` alias so a
    /// renamed import cannot disguise its real root.
    fn classify_segments(&mut self, segments: &[String], ctx: &FileCtx) {
        if segments.is_empty() {
            return;
        }

        let location = self.rel(&ctx.path);
        let Some(resolved) = self.expand_aliases(segments, ctx) else {
            self.violation(
                Category::UnresolvedPath,
                location,
                "alias chain is too deep to resolve and is refused",
            );
            return;
        };

        let root = &resolved[0];

        // A module declared inside this crate is a local path root, not a dependency.
        if self.local_modules.contains(root) {
            return;
        }

        let spelling = resolved.join("::");
        let crate_like = self.is_crate_like(root);

        // A single identifier is a local binding, item or type. It is only interesting
        // when it names a dependency root, which is how `use libloading;` is caught.
        if resolved.len() == 1 && !crate_like {
            return;
        }

        match self.root_class(root, ctx.test_context) {
            RootClass::Allowed => {}
            RootClass::Workspace(package) => {
                let Some(name) = resolved.get(1) else {
                    return;
                };
                if name == "self" {
                    return;
                }
                let allowed = self
                    .spec
                    .surfaces
                    .get(&package)
                    .is_some_and(|surface| surface.names.contains(name));
                if !allowed {
                    self.violation(
                        Category::PrivateCrateInternal,
                        location,
                        format!("`{spelling}` reaches a private internal of {package}"),
                    );
                }
            }
            RootClass::Refused(category, reason) => {
                self.violation(category, location, format!("{reason} (`{spelling}`)"));
            }
        }
    }

    /// Substitutes `use`/`extern crate` aliases until the path starts at a real root.
    ///
    /// `use forbidden::Thing as helper; helper::new()` therefore resolves to
    /// `forbidden::Thing::new`, and a type imported by name keeps its owning crate so a
    /// later `Type::method()` is attributed correctly.
    fn expand_aliases(&self, segments: &[String], ctx: &FileCtx) -> Option<Vec<String>> {
        let mut current: Vec<String> = segments.to_vec();
        for _ in 0..8 {
            let Some(target) = current.first().and_then(|root| ctx.aliases.get(root)) else {
                return Some(current);
            };
            let mut next = target.clone();
            next.extend_from_slice(&current[1..]);
            current = next;
        }
        None
    }

    /// Whether a bare identifier could name a dependency root rather than a local item.
    fn is_crate_like(&self, root: &str) -> bool {
        DYNAMIC_LOADING_ROOTS.contains(&root)
            || self.spec.workspace_roots.contains_key(root)
            || self.spec.known_crates.contains(root)
            || self.spec.allowed_external.contains(root)
            || self.spec.dev_roots.contains(root)
            || graph::APPROVED_EXTERNAL_DEV.contains(&root)
    }
}

enum RootClass {
    Allowed,
    Workspace(String),
    Refused(Category, String),
}

/// The inward-only rule for one workspace→workspace reference, expressed once: the same
/// function the Cargo graph guard uses, so the two layers cannot disagree.
fn allowed_workspace_dep(from: &str, to: &str) -> bool {
    graph::edge_allowed(&graph::Edge {
        from: from.to_owned(),
        to: to.to_owned(),
        kind: DepKind::Normal,
        alias: None,
    })
}

impl Scanner<'_> {
    /// Classifies the first segment of an already alias-expanded path. Anything
    /// unresolvable is refused: an alias or re-export never becomes an implicit
    /// permission.
    fn root_class(&self, root: &str, test_context: bool) -> RootClass {
        if matches!(root, "crate" | "self" | "super" | "Self") {
            return RootClass::Allowed;
        }
        if matches!(root, "std" | "core" | "alloc" | "proc_macro") {
            return RootClass::Allowed;
        }
        if DYNAMIC_LOADING_ROOTS.contains(&root) {
            return RootClass::Refused(
                Category::DynamicLoading,
                "dynamic loading is not inspectable and is forbidden".to_owned(),
            );
        }

        if let Some(package) = self.spec.workspace_roots.get(root) {
            // A test target naming its own library crate is self-reference, not a
            // dependency edge, so the layer rule does not apply to it.
            if package == &self.spec.package {
                return RootClass::Allowed;
            }
            if !allowed_workspace_dep(&self.spec.package, package) {
                return RootClass::Refused(
                    Category::LayerOutwardReference,
                    format!("{} may not depend on {package}", self.spec.package),
                );
            }
            return RootClass::Workspace(package.to_owned());
        }

        if self.spec.known_crates.contains(root) {
            return RootClass::Refused(
                Category::UndeclaredCrate,
                format!(
                    "{root} is not a declared dependency of {}",
                    self.spec.package
                ),
            );
        }
        if self.spec.allowed_external.contains(root) {
            return RootClass::Allowed;
        }
        if test_context && self.spec.dev_roots.contains(root) {
            return RootClass::Allowed;
        }
        if self.spec.dev_roots.contains(root) {
            return RootClass::Refused(
                Category::UnapprovedExternalCrate,
                format!("{root} is a development-only dependency and is not usable here"),
            );
        }
        if graph::APPROVED_EXTERNAL_DEV.contains(&root) {
            return RootClass::Refused(
                Category::UnapprovedExternalCrate,
                format!("{root} is not declared for {}", self.spec.package),
            );
        }
        if PRIMITIVE_ROOTS.contains(&root) {
            return RootClass::Allowed;
        }
        if root.chars().next().is_some_and(char::is_uppercase) {
            return RootClass::Allowed;
        }

        RootClass::Refused(
            Category::UnresolvedPath,
            format!("path root `{root}` resolves to no declared dependency or local item"),
        )
    }
}

/// Everything one item (or one parsed macro argument list) contributes to the check.
#[derive(Default)]
struct Facts {
    paths: Vec<syn::Path>,
    use_paths: Vec<Vec<String>>,
    /// Imported name -> the full used path, so an alias resolves to its real root and a
    /// re-exported type keeps its owning crate.
    aliases: BTreeMap<String, Vec<String>>,
    macros: Vec<syn::Macro>,
    unsafe_found: bool,
    foreign_found: bool,
    verbatim_found: bool,
}

impl Facts {
    fn record_use(&mut self, tree: &syn::UseTree, prefix: &mut Vec<String>) {
        match tree {
            syn::UseTree::Path(segment) => {
                prefix.push(segment.ident.to_string());
                self.record_use(&segment.tree, prefix);
                prefix.pop();
            }
            syn::UseTree::Name(name) => {
                let mut segments = prefix.clone();
                segments.push(name.ident.to_string());
                self.record_leaf(&segments);
            }
            syn::UseTree::Rename(rename) => {
                let mut segments = prefix.clone();
                segments.push(rename.ident.to_string());
                // `use forbidden::Thing as helper;` binds `helper` to the real path, so
                // the disguise resolves to the forbidden crate.
                self.aliases
                    .insert(rename.rename.to_string(), segments.clone());
                self.use_paths.push(segments);
            }
            syn::UseTree::Glob(_) => {
                if !prefix.is_empty() {
                    self.use_paths.push(prefix.clone());
                }
            }
            syn::UseTree::Group(group) => {
                for item in &group.items {
                    self.record_use(item, prefix);
                }
            }
        }
    }

    fn record_leaf(&mut self, segments: &[String]) {
        if let Some(leaf) = segments.last() {
            self.aliases.insert(leaf.clone(), segments.to_vec());
        }
        self.use_paths.push(segments.to_vec());
    }
}

impl<'ast> Visit<'ast> for Facts {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        self.paths.push(node.clone());
        syn::visit::visit_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.macros.push(node.clone());
        syn::visit::visit_macro(self, node);
    }

    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        let mut prefix: Vec<String> = Vec::new();
        self.record_use(&node.tree, &mut prefix);
        syn::visit::visit_item_use(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'ast syn::ItemExternCrate) {
        let target = node.ident.to_string();
        let local = node
            .rename
            .as_ref()
            .map_or_else(|| target.clone(), |(_, alias)| alias.to_string());
        self.aliases.insert(local, vec![target]);
        syn::visit::visit_item_extern_crate(self, node);
    }

    fn visit_expr_unsafe(&mut self, node: &'ast syn::ExprUnsafe) {
        self.unsafe_found = true;
        syn::visit::visit_expr_unsafe(self, node);
    }

    fn visit_item_foreign_mod(&mut self, node: &'ast syn::ItemForeignMod) {
        self.foreign_found = true;
        syn::visit::visit_item_foreign_mod(self, node);
    }

    fn visit_item(&mut self, node: &'ast syn::Item) {
        if matches!(node, syn::Item::Verbatim(_)) {
            self.verbatim_found = true;
        }
        match node {
            syn::Item::Fn(item) if matches!(item.sig.safety, syn::Safety::Unsafe(_)) => {
                self.unsafe_found = true;
            }
            syn::Item::Impl(item) if item.unsafety.is_some() => self.unsafe_found = true,
            syn::Item::Trait(item) if item.unsafety.is_some() => self.unsafe_found = true,
            _ => {}
        }
        syn::visit::visit_item(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        if matches!(node.sig.safety, syn::Safety::Unsafe(_)) {
            self.unsafe_found = true;
        }
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast syn::Attribute) {
        // `#[derive(...)]` item lists are not visited as paths by `syn`, so they are
        // parsed explicitly; a derive naming a forbidden crate is then classified.
        if node.path().is_ident("derive")
            && let syn::Meta::List(list) = &node.meta
        {
            use syn::parse::Parser;
            if let Ok(paths) =
                syn::punctuated::Punctuated::<syn::Path, syn::token::Comma>::parse_terminated
                    .parse2(list.tokens.clone())
            {
                self.paths.extend(paths);
            }
        }
        syn::visit::visit_attribute(self, node);
    }
}

/// True when a `#[cfg(...)]` predicate mentions the `test` word.
fn cfg_is_test(attributes: &[syn::Attribute]) -> bool {
    attributes.iter().any(|attribute| {
        if !attribute.path().is_ident("cfg") {
            return false;
        }
        let syn::Meta::List(list) = &attribute.meta else {
            return false;
        };
        list.tokens
            .to_string()
            .split(|character: char| !character.is_alphanumeric() && character != '_')
            .any(|word| word == "test")
    })
}

/// How the source file of an out-of-line module is selected.
///
/// The checker refuses any selection it cannot pin down: a conditional `#[path]` could
/// choose a different file on another platform, and inspecting only the default while the
/// compiler selects an alternative would be a silent miss.
enum ModulePathSelection {
    /// No `#[path]` attribute: resolve `name.rs` / `name/mod.rs`.
    Default,
    /// Exactly one literal `#[path = "..."]`.
    Literal(String),
    /// Conditional, malformed or duplicated selection: refuse, never guess.
    Refused(Category, String),
}

fn module_path_selection(attributes: &[syn::Attribute]) -> ModulePathSelection {
    let mut literal: Option<String> = None;
    let mut duplicate = false;

    for attribute in attributes {
        if attribute.path().is_ident("path") {
            let syn::Meta::NameValue(value) = &attribute.meta else {
                return ModulePathSelection::Refused(
                    Category::ModuleSourceUnresolved,
                    "module `#[path]` is malformed and is refused".to_owned(),
                );
            };
            let syn::Expr::Lit(lit) = &value.value else {
                return ModulePathSelection::Refused(
                    Category::ModuleSourceUnresolved,
                    "module `#[path]` is not a literal string and is refused".to_owned(),
                );
            };
            let syn::Lit::Str(text) = &lit.lit else {
                return ModulePathSelection::Refused(
                    Category::ModuleSourceUnresolved,
                    "module `#[path]` is not a string literal and is refused".to_owned(),
                );
            };
            if literal.replace(text.value()).is_some() {
                duplicate = true;
            }
            continue;
        }

        if attribute.path().is_ident("cfg_attr") {
            match cfg_attr_may_select_path(attribute) {
                Ok(false) => {}
                Ok(true) => {
                    return ModulePathSelection::Refused(
                        Category::ConditionalModulePath,
                        "module source may be selected by a conditional `#[cfg_attr(..., path = ...)]` \
                         and is refused rather than inspected on this platform alone"
                            .to_owned(),
                    );
                }
                Err(reason) => {
                    return ModulePathSelection::Refused(Category::ConditionalModulePath, reason);
                }
            }
        }
    }

    if duplicate {
        return ModulePathSelection::Refused(
            Category::ModuleSourceAmbiguous,
            "module declares more than one `#[path]` and is refused".to_owned(),
        );
    }

    match literal {
        Some(path) => ModulePathSelection::Literal(path),
        None => ModulePathSelection::Default,
    }
}

/// Whether a `#[cfg_attr(...)]` can select or obscure a `path`, including nested forms.
fn cfg_attr_may_select_path(attribute: &syn::Attribute) -> Result<bool, String> {
    let syn::Meta::List(list) = &attribute.meta else {
        return Err("`#[cfg_attr]` could not be analysed and is refused".to_owned());
    };
    let metas = parse_meta_list(list).ok_or_else(|| {
        "`#[cfg_attr]` arguments could not be analysed and are refused".to_owned()
    })?;

    for meta in metas.iter().skip(1) {
        if meta_mentions_path(meta)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn meta_mentions_path(meta: &syn::Meta) -> Result<bool, String> {
    if meta.path().is_ident("path") {
        return Ok(true);
    }
    if meta.path().is_ident("cfg_attr") {
        let syn::Meta::List(list) = meta else {
            return Err("nested `cfg_attr` could not be analysed and is refused".to_owned());
        };
        let metas = parse_meta_list(list).ok_or_else(|| {
            "nested `cfg_attr` arguments could not be analysed and are refused".to_owned()
        })?;
        for item in metas.iter().skip(1) {
            if meta_mentions_path(item)? {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

fn parse_meta_list(
    list: &syn::MetaList,
) -> Option<syn::punctuated::Punctuated<syn::Meta, syn::token::Comma>> {
    use syn::parse::Parser;
    syn::punctuated::Punctuated::<syn::Meta, syn::token::Comma>::parse_terminated
        .parse2(list.tokens.clone())
        .ok()
}

fn file_stem(path: &Path) -> String {
    path.file_stem()
        .map(|stem| stem.to_string_lossy().into_owned())
        .unwrap_or_default()
}

/// Lexical normalisation only: no filesystem access, no OS path expansion.
fn normalize_lexical(path: &Path) -> PathBuf {
    let mut parts: Vec<Component<'_>> = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(parts.last(), Some(Component::Normal(_))) {
                    parts.pop();
                } else {
                    parts.push(component);
                }
            }
            other => parts.push(other),
        }
    }
    parts.iter().collect()
}

fn is_inside(path: &Path, root: &Path) -> bool {
    path.starts_with(root)
}

/// Repository-relative label for a package root, e.g. `crates/solo-domain`.
fn label_root(package_root: &Path) -> String {
    let name = package_root
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    format!("crates/{name}")
}

/// Every Rust file under a package root, excluding build output and hidden directories.
///
/// Discovery fails closed: an unreadable directory or any redirect encountered while
/// walking is reported, and a redirect is never descended into, so a junction cannot hide
/// or substitute source. Files reached only through a redirect can therefore never be
/// counted as inspected.
/// Unresolved type of one directory entry, obtained **without** following links.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Dir,
    /// Neither a file nor a directory (socket, device, FIFO…): never silently ignored.
    Other,
}

/// Filesystem inspection seam for package discovery.
///
/// It exists so a deterministic inspection failure can be exercised through the *same*
/// error-handling code the real walker uses, rather than inferring behaviour from a mock.
pub trait DiscoveryFs {
    fn list(&self, directory: &Path) -> std::io::Result<Vec<std::ffi::OsString>>;
    fn kind(&self, path: &Path) -> std::io::Result<EntryKind>;
    fn redirected(&self, path: &Path) -> std::io::Result<bool>;
}

/// The real filesystem inspector: unresolved metadata only.
pub struct RealDiscoveryFs;

impl DiscoveryFs for RealDiscoveryFs {
    fn list(&self, directory: &Path) -> std::io::Result<Vec<std::ffi::OsString>> {
        let entries = std::fs::read_dir(directory)?;
        let mut names = Vec::new();
        for entry in entries {
            names.push(entry?.file_name());
        }
        Ok(names)
    }

    fn kind(&self, path: &Path) -> std::io::Result<EntryKind> {
        let metadata = std::fs::symlink_metadata(path)?;
        let file_type = metadata.file_type();
        Ok(if file_type.is_file() {
            EntryKind::File
        } else if file_type.is_dir() {
            EntryKind::Dir
        } else {
            EntryKind::Other
        })
    }

    fn redirected(&self, path: &Path) -> std::io::Result<bool> {
        Ok(is_redirect(&std::fs::symlink_metadata(path)?))
    }
}
/// Every Rust file under a package root, excluding build output and hidden directories.
///
/// Discovery fails closed. For **every** visited entry — including one whose name is not
/// `.rs` — the unresolved type and the redirect status are established explicitly: an
/// inspection error becomes a refusal, a redirect is reported and never descended into, and
/// an entry that is neither a file nor a directory is refused rather than dropped. Ignored
/// names (`target`, hidden directories) are skipped only *after* their identity has been
/// inspected safely.
pub fn rust_files_under(
    root: &Path,
    label: &str,
    anchor: &Path,
    fs: &dyn DiscoveryFs,
) -> (Vec<PathBuf>, Vec<Violation>) {
    let mut found: Vec<PathBuf> = Vec::new();
    let mut violations: Vec<Violation> = Vec::new();
    let mut pending: Vec<PathBuf> = vec![normalize_lexical(root)];

    while let Some(directory) = pending.pop() {
        let names = match fs.list(&directory) {
            Ok(names) => names,
            Err(_) => {
                violations.push(Violation::new(
                    Category::UnreadableSource,
                    format!("{label}/{}", relative_label(root, &directory)),
                    "package directory could not be listed and cannot be audited",
                ));
                continue;
            }
        };

        for name in names {
            let path = directory.join(&name);
            let display = format!("{label}/{}", relative_label(root, &path));
            let name_text = name.to_string_lossy().into_owned();

            let redirected = match fs.redirected(&path) {
                Ok(redirected) => redirected,
                Err(_) => {
                    violations.push(Violation::new(
                        Category::UnreadableSource,
                        display.clone(),
                        "package entry metadata could not be inspected",
                    ));
                    continue;
                }
            };
            if redirected {
                violations.push(Violation::new(
                    Category::ModuleSourceEscape,
                    display.clone(),
                    "package entry is a link or reparse point and is not audited",
                ));
                continue;
            }

            let kind = match fs.kind(&path) {
                Ok(kind) => kind,
                Err(_) => {
                    violations.push(Violation::new(
                        Category::UnreadableSource,
                        display.clone(),
                        "package entry type could not be established",
                    ));
                    continue;
                }
            };

            match kind {
                EntryKind::Dir => {
                    if name_text == "target" || name_text.starts_with('.') {
                        continue;
                    }
                    if let Err(error) = inspect_original_chain(&path) {
                        let (category, detail) = match error {
                            ConfinementError::Escape => (
                                Category::ModuleSourceEscape,
                                "package directory is reached through a redirect and is refused",
                            ),
                            ConfinementError::Unresolvable => (
                                Category::UnreadableSource,
                                "package directory could not be inspected before descending",
                            ),
                        };
                        violations.push(Violation::new(category, display.clone(), detail));
                        continue;
                    }
                    if !is_inside(&normalize_lexical(&path), &normalize_lexical(anchor)) {
                        violations.push(Violation::new(
                            Category::ModuleSourceEscape,
                            display.clone(),
                            "package directory lies outside the checkout anchor",
                        ));
                        continue;
                    }
                    pending.push(path);
                }
                EntryKind::File => {
                    if name_text.ends_with(".rs") {
                        found.push(normalize_lexical(&path));
                    }
                }
                EntryKind::Other => {
                    violations.push(Violation::new(
                        Category::UnreadableSource,
                        display.clone(),
                        "package entry is neither a file nor a directory and cannot be audited",
                    ));
                }
            }
        }
    }

    found.sort();
    (found, violations)
}

fn relative_label(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .map(|tail| tail.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|_| "<outside>".to_owned())
}

/// Public root surface of each workspace library crate: the names another crate may
/// legally write after the crate root.
pub fn public_surfaces(
    packages: &[PackageTargets],
    reader: &dyn SourceReader,
    anchor: &Path,
) -> (BTreeMap<String, PublicSurface>, Vec<Violation>) {
    let mut surfaces: BTreeMap<String, PublicSurface> = BTreeMap::new();
    let mut violations: Vec<Violation> = Vec::new();

    for package in packages {
        if package.lib_crate.is_none() {
            continue;
        }
        let Some(target) = package
            .targets
            .iter()
            .find(|target| target.kind == TargetKind::Lib)
        else {
            continue;
        };
        let root = normalize_lexical(&target.src_path);
        if let Err(error) = reader.confine(&root, &package.root, anchor) {
            let (category, detail) = match error {
                ConfinementError::Escape => (
                    Category::ModuleSourceEscape,
                    "library root is redirected outside its package and is refused",
                ),
                ConfinementError::Unresolvable => (
                    Category::UnreadableSource,
                    "library root could not be proven confined while computing the public surface",
                ),
            };
            violations.push(Violation::new(
                category,
                format!("{}/src/<lib>", label_root(&package.root)),
                detail,
            ));
            continue;
        }
        let Some(text) = reader.read(&root) else {
            violations.push(Violation::new(
                Category::UnreadableSource,
                format!("{}/src/<lib>", label_root(&package.root)),
                "library root could not be read while computing the public surface",
            ));
            continue;
        };
        let Ok(file) = syn::parse_file(&text) else {
            violations.push(Violation::new(
                Category::UnparsableSource,
                format!("{}/src/<lib>", label_root(&package.root)),
                "library root is not parsable while computing the public surface",
            ));
            continue;
        };

        let lib_dir = root.parent().map(Path::to_path_buf).unwrap_or_default();
        let mut pass = SurfacePass {
            lib_dir,
            reader,
            package_root: package.root.clone(),
            anchor: anchor.to_path_buf(),
            surface: PublicSurface::default(),
            violations: Vec::new(),
        };
        collect_public(&file.items, &mut pass);
        let SurfacePass {
            surface,
            violations: found,
            ..
        } = pass;
        violations.extend(found);
        // Keyed by package name, the identity the layer and privacy rules use.
        surfaces.insert(package.name.clone(), surface);
    }

    (surfaces, violations)
}

fn public_item_name(item: &syn::Item) -> Option<String> {
    use syn::Visibility::Public;

    match item {
        syn::Item::Struct(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Enum(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Union(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Fn(item) if matches!(item.vis, Public(_)) => Some(item.sig.ident.to_string()),
        syn::Item::Trait(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Type(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Const(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Static(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        syn::Item::Mod(item) if matches!(item.vis, Public(_)) => Some(item.ident.to_string()),
        _ => None,
    }
}

fn surface_violation(package_root: &Path, detail: &str) -> Violation {
    Violation::new(
        Category::UnresolvedPath,
        format!("{}/src", label_root(package_root)),
        detail,
    )
}

/// Accumulated state of one package's public-surface walk.
///
/// Bundled so the recursive walkers stay well under the argument limit while still
/// carrying the anchored confinement context.
struct SurfacePass<'a> {
    lib_dir: PathBuf,
    reader: &'a dyn SourceReader,
    package_root: PathBuf,
    anchor: PathBuf,
    surface: PublicSurface,
    violations: Vec<Violation>,
}

fn collect_public(items: &[syn::Item], pass: &mut SurfacePass<'_>) {
    for item in items {
        if let syn::Item::Use(node) = item
            && matches!(node.vis, syn::Visibility::Public(_))
        {
            collect_public_use(&node.tree, &mut Vec::new(), pass);
            continue;
        }
        if let Some(name) = public_item_name(item) {
            pass.surface.names.insert(name);
        }
    }
}

fn collect_public_use(tree: &syn::UseTree, prefix: &mut Vec<String>, pass: &mut SurfacePass<'_>) {
    match tree {
        syn::UseTree::Path(segment) => {
            prefix.push(segment.ident.to_string());
            collect_public_use(&segment.tree, prefix, pass);
            prefix.pop();
        }
        syn::UseTree::Name(name) => {
            pass.surface.names.insert(name.ident.to_string());
        }
        syn::UseTree::Rename(rename) => {
            pass.surface.names.insert(rename.rename.to_string());
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                collect_public_use(item, prefix, pass);
            }
        }
        syn::UseTree::Glob(_) => {
            // A globbed surface is usable only when the module can be located and
            // inspected **and** its file is confined; otherwise the re-export is refused.
            let package_root = pass.package_root.clone();
            let Some(path) = resolve_module_file(prefix, &pass.lib_dir) else {
                pass.violations.push(surface_violation(
                    &package_root,
                    "public glob re-export does not resolve to a module and is refused",
                ));
                return;
            };
            let parsed = match pass.reader.confine(&path, &package_root, &pass.anchor) {
                Ok(_) => std::fs::read_to_string(&path).map(|text| syn::parse_file(&text)),
                Err(_) => {
                    pass.violations.push(surface_violation(
                        &package_root,
                        "public glob re-export target is not confined and is refused",
                    ));
                    return;
                }
            };
            let Ok(Ok(file)) = parsed else {
                pass.violations.push(surface_violation(
                    &package_root,
                    "public glob re-export target is not parsable and is refused",
                ));
                return;
            };
            for item in &file.items {
                if let syn::Item::Use(node) = item
                    && matches!(node.vis, syn::Visibility::Public(_))
                {
                    pass.violations.push(surface_violation(
                        &package_root,
                        "nested glob re-export cannot be resolved and is refused",
                    ));
                    continue;
                }
                if let Some(name) = public_item_name(item) {
                    pass.surface.names.insert(name);
                }
            }
        }
    }
}

fn resolve_module_file(prefix: &[String], lib_dir: &Path) -> Option<PathBuf> {
    let segments: Vec<&str> = prefix
        .iter()
        .map(String::as_str)
        .filter(|segment| !matches!(*segment, "crate" | "self" | "super"))
        .collect();
    if segments.is_empty() {
        return None;
    }

    let mut path = lib_dir.to_path_buf();
    for segment in segments {
        path = path.join(segment);
    }

    let file = path.with_extension("rs");
    if file.is_file() {
        return Some(file);
    }
    let module = path.join("mod.rs");
    if module.is_file() {
        return Some(module);
    }
    None
}

/// Deterministic synthetic workspace used only by the negative fixtures. It is a
/// declared-dependency table and surface map, never a second checkout.
pub struct FixtureWorld {
    pub workspace_roots: BTreeMap<String, String>,
    pub known_crates: BTreeSet<String>,
    pub surfaces: BTreeMap<String, PublicSurface>,
}

impl FixtureWorld {
    pub fn standard() -> Self {
        let mut workspace_roots: BTreeMap<String, String> = BTreeMap::new();
        for (root, package) in [
            ("solo_domain", "solo-domain"),
            ("solo_application", "solo-application"),
            ("solo_adapters", "solo-adapters"),
        ] {
            workspace_roots.insert(root.to_owned(), package.to_owned());
        }

        let mut surfaces: BTreeMap<String, PublicSurface> = BTreeMap::new();
        surfaces.insert(
            "solo-domain".to_owned(),
            PublicSurface::of(&["BaseCandidateV1", "EntityIdV1"]),
        );
        surfaces.insert(
            "solo-application".to_owned(),
            PublicSurface::of(&["BootstrapPreflightService", "BootstrapInputV1"]),
        );
        // `bootstrap_cli` is deliberately absent: it is a private adapter internal.
        surfaces.insert(
            "solo-adapters".to_owned(),
            PublicSurface::of(&["CliOutcome", "run"]),
        );

        Self {
            workspace_roots,
            known_crates: [
                "solo_domain",
                "solo_application",
                "solo_adapters",
                "solo_cli",
            ]
            .iter()
            .map(|name| (*name).to_owned())
            .collect(),
            surfaces,
        }
    }

    /// Builds a spec for a synthetic package root at `fixture/<package>/src/<file>`.
    pub fn spec(
        &self,
        package: &str,
        kind: TargetKind,
        file: &str,
        dev_roots: &[&str],
    ) -> UnitSpec {
        let package_root = PathBuf::from("fixture").join(package.replace('-', "_"));
        let dev: BTreeSet<String> = dev_roots.iter().map(|name| (*name).to_owned()).collect();
        let allowed_external = if kind.is_test_context() {
            dev.clone()
        } else {
            BTreeSet::new()
        };

        UnitSpec {
            package: package.to_owned(),
            kind,
            root_file: package_root.join("src").join(file),
            package_root,
            anchor: PathBuf::from("fixture"),
            workspace_roots: self.workspace_roots.clone(),
            allowed_external,
            dev_roots: dev,
            known_crates: self.known_crates.clone(),
            surfaces: self.surfaces.clone(),
            label_root: "fixture".to_owned(),
        }
    }
}
