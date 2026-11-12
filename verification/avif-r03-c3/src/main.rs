//! Driver for the standalone AVIF qualification probe (non-product evidence).
//!
//! Two roles:
//!   * `--all`   — parent: runs every case in its own OS process with a hard
//!                 wall-clock timeout, collects one JSON line per case, and fails
//!                 only on a genuine functional assertion (never on an honest
//!                 `BLOCKED`/`UNVERIFIED` resource finding).
//!   * `--case`  — child: evaluates exactly one case and writes its record.
//!
//! `--list` prints the case identifiers so the workflow can enumerate them.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use solo_avif_r03_c3_proof as probe;

fn main() {
    let code = run();
    std::process::exit(code);
}

fn run() -> i32 {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut mode = String::from("all");
    let mut case_id: Option<String> = None;
    let mut pkg: Option<PathBuf> = std::env::var_os("OXIDEAV_AVIF_PKG_DIR").map(PathBuf::from);
    let mut out: Option<PathBuf> = None;
    let mut json_out: Option<PathBuf> = None;
    let mut timeout_ms: u64 = probe::DEFAULT_CASE_TIMEOUT_MS;

    let mut i = 0usize;
    while i < args.len() {
        match args[i].as_str() {
            "--list" => mode = String::from("list"),
            "--all" => mode = String::from("all"),
            "--case" => {
                i += 1;
                case_id = args.get(i).cloned();
                mode = String::from("case");
            }
            "--pkg" => {
                i += 1;
                pkg = args.get(i).map(PathBuf::from);
            }
            "--out" => {
                i += 1;
                out = args.get(i).map(PathBuf::from);
            }
            "--json" => {
                i += 1;
                json_out = args.get(i).map(PathBuf::from);
            }
            "--timeout-ms" => {
                i += 1;
                timeout_ms = args
                    .get(i)
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(probe::DEFAULT_CASE_TIMEOUT_MS);
            }
            other => {
                eprintln!("unknown-argument: {other}");
                return 2;
            }
        }
        i += 1;
    }

    let pkg_dir = match probe::locate_package(pkg.as_deref()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("package-locate-failed: {e}");
            return 3;
        }
    };
    let fixtures = match probe::discover_fixtures(&pkg_dir) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("fixture-discovery-failed: {e}");
            return 3;
        }
    };
    let cases = probe::build_cases(&fixtures);

    match mode.as_str() {
        "list" => {
            for c in cases.iter() {
                println!("{}", c.id);
            }
            0
        }
        "case" => run_single(&cases, &fixtures, &pkg_dir, case_id.as_deref(), out, json_out),
        _ => {
            let out_dir = out.unwrap_or_else(|| PathBuf::from("avif-out"));
            run_all(&cases, &fixtures, &pkg_dir, &out_dir, timeout_ms)
        }
    }
}

fn base_fixture(fixtures: &[String]) -> String {
    for want in ["tests/fixtures/black420.avif", "tests/fixtures/red.avif"] {
        if let Some(f) = fixtures.iter().find(|f| f.as_str() == want) {
            return f.clone();
        }
    }
    fixtures
        .first()
        .cloned()
        .unwrap_or_else(|| "tests/fixtures/red.avif".to_string())
}

fn load_case_bytes(case: &probe::Case, pkg: &Path, fixtures: &[String]) -> Result<Vec<u8>, String> {
    if let Some(rel) = case.id.strip_prefix("pos:") {
        std::fs::read(pkg.join(rel)).map_err(|e| format!("read fixture {rel}: {e}"))
    } else {
        let base_rel = base_fixture(fixtures);
        let base = std::fs::read(pkg.join(&base_rel))
            .map_err(|e| format!("read base fixture {base_rel}: {e}"))?;
        probe::build_negative(&case.id, &base)
    }
}

fn run_single(
    cases: &[probe::Case],
    fixtures: &[String],
    pkg: &Path,
    case_id: Option<&str>,
    out: Option<PathBuf>,
    json_out: Option<PathBuf>,
) -> i32 {
    let id = match case_id {
        Some(v) => v,
        None => {
            eprintln!("missing --case <ID>");
            return 2;
        }
    };
    let case = match cases.iter().find(|c| c.id == id) {
        Some(c) => c,
        None => {
            eprintln!("unknown-case: {id}");
            return 2;
        }
    };
    let out_dir = out.unwrap_or_else(|| PathBuf::from("avif-out"));
    if std::fs::create_dir_all(&out_dir).is_err() {
        eprintln!("cannot-create-output-dir");
        return 3;
    }
    let bytes = match load_case_bytes(case, pkg, fixtures) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("input-build-failed: {e}");
            return 4;
        }
    };
    let rec = probe::evaluate_case(case, &bytes, Some(&out_dir));
    let line = rec.to_json_line();
    if let Some(path) = json_out {
        if std::fs::write(&path, format!("{line}\n")).is_err() {
            eprintln!("cannot-write-json-record");
            return 5;
        }
    }
    println!("{}", rec.summary_line());
    0
}

#[cfg(target_os = "linux")]
fn read_peak_rss(pid: u32) -> Option<u64> {
    let status = std::fs::read_to_string(format!("/proc/{pid}/status")).ok()?;
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmHWM:") {
            let kb: u64 = rest.trim().trim_end_matches("kB").trim().parse().ok()?;
            return Some(kb.saturating_mul(1024));
        }
    }
    None
}

#[cfg(not(target_os = "linux"))]
fn read_peak_rss(_pid: u32) -> Option<u64> {
    None
}

fn json_status(line: &str) -> &'static str {
    if line.contains("\"status\":\"PASS\"") {
        "PASS"
    } else if line.contains("\"status\":\"UNVERIFIED\"") {
        "UNVERIFIED"
    } else {
        "FAIL"
    }
}

fn run_all(
    cases: &[probe::Case],
    fixtures: &[String],
    pkg: &Path,
    out_dir: &Path,
    timeout_ms: u64,
) -> i32 {
    if std::fs::create_dir_all(out_dir).is_err() {
        eprintln!("cannot-create-output-dir");
        return 3;
    }
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cannot-resolve-current-exe: {e}");
            return 3;
        }
    };

    let mut jsonl = String::new();
    let mut pass = 0usize;
    let mut unverified = 0usize;
    let mut fail = 0usize;

    // A fixture that silently disappears must fail the run, not shrink the proof.
    for expected in probe::EXPECTED_FIXTURES.iter() {
        if !fixtures.iter().any(|f| f == expected) {
            println!("pos:{} status=FAIL result=FIXTURE_MISSING", expected);
            jsonl.push_str(&format!(
                "{{\"case_id\":\"pos:{expected}\",\"kind\":\"positive\",\"expectation\":\"must_decode\",\"result\":\"FIXTURE_MISSING\",\"status\":\"FAIL\"}}\n"
            ));
            fail += 1;
        }
    }

    println!(
        "probe-cases={} fixtures={} timeout_ms={} host={}",
        cases.len(),
        fixtures.len(),
        timeout_ms,
        std::env::consts::OS
    );

    for case in cases.iter() {
        let json_path = out_dir.join(format!("{}.json", probe::sanitize_id(&case.id)));
        let _ = std::fs::remove_file(&json_path);

        let mut cmd = Command::new(&exe);
        cmd.arg("--case")
            .arg(&case.id)
            .arg("--pkg")
            .arg(pkg)
            .arg("--out")
            .arg(out_dir)
            .arg("--json")
            .arg(&json_path);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let start = Instant::now();
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                println!("{} status=FAIL result=SPAWN_FAILED err={}", case.id, e);
                jsonl.push_str(&format!(
                    "{{\"case_id\":\"{}\",\"kind\":\"{}\",\"expectation\":\"{}\",\"result\":\"SPAWN_FAILED\",\"status\":\"FAIL\"}}\n",
                    case.id,
                    case.kind.label(),
                    case.kind.expectation()
                ));
                fail += 1;
                continue;
            }
        };
        let pid = child.id();
        let mut peak: Option<u64> = None;
        let mut timed_out = false;
        loop {
            match child.try_wait() {
                Ok(Some(_)) => break,
                Ok(None) => {
                    if let Some(p) = read_peak_rss(pid) {
                        peak = Some(peak.map_or(p, |cur: u64| cur.max(p)));
                    }
                    if start.elapsed().as_millis() as u64 >= timeout_ms {
                        let _ = child.kill();
                        let _ = child.wait();
                        timed_out = true;
                        break;
                    }
                    std::thread::sleep(Duration::from_millis(20));
                }
                Err(_) => break,
            }
        }
        let wall_ms = start.elapsed().as_millis() as u64;

        let mut line = match std::fs::read_to_string(&json_path) {
            Ok(s) => s.trim().to_string(),
            Err(_) => String::new(),
        };
        if line.is_empty() {
            let result = if timed_out { "TIMEOUT" } else { "NO_OUTPUT" };
            line = format!(
                "{{\"case_id\":\"{}\",\"kind\":\"{}\",\"expectation\":\"{}\",\"result\":\"{}\",\"status\":\"FAIL\",\"note\":\"child produced no record\",\"wall_ms\":{},\"timed_out\":{}}}",
                case.id,
                case.kind.label(),
                case.kind.expectation(),
                result,
                wall_ms,
                timed_out
            );
        }

        let status = json_status(&line);
        match status {
            "PASS" => pass += 1,
            "UNVERIFIED" => unverified += 1,
            _ => fail += 1,
        }
        println!("record {line}");
        println!(
            "annotation case={} wall_ms={} peak_rss_bytes={} timed_out={} status={}",
            case.id,
            wall_ms,
            match peak {
                Some(v) => v.to_string(),
                None => "unavailable".to_string(),
            },
            timed_out,
            status
        );
        if timed_out {
            println!("{} status=FAIL result=TIMEOUT_EXCEEDED budget_ms={}", case.id, timeout_ms);
        }
        jsonl.push_str(&line);
        jsonl.push('\n');
    }

    let results = out_dir.join("results.jsonl");
    if std::fs::write(&results, &jsonl).is_err() {
        eprintln!("cannot-write-results-jsonl");
        return 3;
    }
    println!(
        "TOTAL pass={} unverified={} fail={} results_jsonl={}",
        pass,
        unverified,
        fail,
        results.display()
    );

    if fail > 0 { 1 } else { 0 }
}
