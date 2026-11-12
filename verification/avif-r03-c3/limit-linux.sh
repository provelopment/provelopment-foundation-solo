#!/usr/bin/env bash
# Linux containment launcher and test driver for the standalone AVIF proof.
#
# NON-PRODUCT EVIDENCE for `SOLO-WO-070 R01`. Nothing here is product code and
# nothing here approves a decoder.
#
# What it does: before the contained program executes a single instruction it
# installs *hard* kernel limits with `ulimit` (RLIMIT_AS and RLIMIT_CPU, soft and
# hard), starts the program as a new session/process-group leader, and keeps a
# finite wall-clock watchdog that terminates the whole group. On any violation it
# returns a typed refusal, discards every staged output file and promotes nothing.
#
# RLIMIT_AS is virtual-address-space enforcement. It is NOT an RSS measurement, and
# the /proc peak figures reported by earlier runs are NOT treated as a cap.
#
# Usage (single contained run):
#   limit-linux.sh run --mem-kib N --cpu-sec N --wall-ms N \
#     [--stage DIR] [--final DIR] [--json FILE] [--input FILE] \
#     [--expect-sha256 HEX] [--case-file NAME] [--test ID] -- CMD [ARGS...]
#
# Usage (full isolation suite):
#   limit-linux.sh suite --bin PATH --pkg DIR --root DIR --jsonl FILE
#
# Exit status: 0 when the run/test outcome matched what the caller required,
# 1 when a required assertion failed, 2 on usage error.
set -uo pipefail

MEM_KIB=524288      # 512 MiB == 524288 KiB (RLIMIT_AS unit)
CPU_SEC=10
WALL_MS=15000
MAX_INPUT_BYTES=20971520

usage() {
  echo "usage: $0 run|suite [options]" >&2
  exit 2
}

now_ms() {
  # EPOCHREALTIME is bash>=5 and avoids spawning date(1) in the watchdog loop.
  local t=${EPOCHREALTIME/,/.}
  local sec=${t%.*}
  local frac=${t#*.}
  echo $(( sec * 1000 + 10#${frac:0:3} ))
}

json_escape() {
  local s=$1
  s=${s//\\/\\\\}
  s=${s//\"/\\\"}
  s=${s//$'\n'/\\n}
  s=${s//$'\r'/}
  s=${s//$'\t'/ }
  printf '%s' "$s"
}

# CPU seconds consumed by this shell's reaped children. `times` must be captured
# WITHOUT a subshell (a command substitution would report the subshell's own empty
# child accounting), so the caller redirects it to a file and passes that file here.
child_cpu_seconds() {
  awk '
    NR == 2 {
      u = $1; s = $2
      sub(/s$/, "", u); sub(/s$/, "", s)
      split(u, a, "m"); split(s, b, "m")
      printf "%.3f", (a[1] * 60 + a[2]) + (b[1] * 60 + b[2])
    }' "$1"
}

# Enforceability probe: can these limits be installed, and are they un-raiseable?
# Runs in a subshell so the caller's own limits are untouched. The soft limit must
# be lowered before the hard limit: setting a hard limit below the current soft
# limit is rejected with EINVAL by the kernel.
limits_installable() {
  ( ulimit -Sv "$1" 2>/dev/null &&
    ulimit -Hv "$1" 2>/dev/null &&
    ulimit -St "$2" 2>/dev/null &&
    ulimit -Ht "$2" 2>/dev/null ) || return 1
  # An unprivileged process must NOT be able to raise the hard limit again.
  if ( ulimit -Hv "$1" 2>/dev/null && ulimit -Hv "$(( $1 + 1024 ))" 2>/dev/null ); then
    return 1
  fi
  return 0
}

field() {
  # field <jsonfile> <key>  -> first matching value, unquoted
  sed -n "s/.*\"$2\":\(\"[^\"]*\"\|[0-9][0-9.]*\).*/\1/p" "$1" | head -n1 | tr -d '"'
}

emit_record() {
  # $1 json file, $2 record body (already JSON fields, no braces)
  printf '{%s}\n' "$2" >> "$1"
}

suite_mode() {
  local bin="" pkg="" root="" jsonl=""
  while (( $# > 0 )); do
    case "$1" in
      --bin) bin=$2; shift 2 ;;
      --pkg) pkg=$2; shift 2 ;;
      --root) root=$2; shift 2 ;;
      --jsonl) jsonl=$2; shift 2 ;;
      *) usage ;;
    esac
  done
  [[ -n $bin && -n $pkg && -n $root && -n $jsonl ]] || usage
  [[ -x $bin ]] || { echo "suite: proof binary not executable: $(basename "$bin")" >&2; exit 2; }
  mkdir -p "$root"
  : > "$jsonl"
  local me="$0"
  local PASSED=0 FAILED=0
  local NORMAL_MEM=524288 NORMAL_CPU=10 NORMAL_WALL=15000

  # Known decoded-plane digests from the accepted WO-069 corpus. A promoted plane
  # file must reproduce them exactly under isolation.
  local -A KNOWN=(
    [pos:tests/fixtures/alpha_video.avif]=3d26373d70bb0b2f0ced1ec923f1f51a592dd45e6e05706569e9994714e6b123
    [pos:tests/fixtures/bbb_alpha.avif]=ad25ae289b08e19c58b27a60a538f82e2085876ee9aa265a922bdffe10844f6f
    [pos:tests/fixtures/black420.avif]=d7928dbf7570fb114f49e426af9f36e70a9b4450b493cfbf6767826ab1cf63bb
    [pos:tests/fixtures/gray32.avif]=ca707edbab56569fcbef9523c0a6661caf2de0f0b7e205169ba2a246a3a3f3d5
    [pos:tests/fixtures/midgray.avif]=b4052e7408e5fa2bc3bdcd8aab8d1f85893825d36bb13b13f1112e6b7fce6d8a
    [pos:tests/fixtures/red.avif]=205691271feb77cc0f208f495bf466e2fb9a9733d816b2d2355f05eb6c15552c
    [pos:tests/fixtures/white16.avif]=dec238c044dec03455ecb89858492569cbd64026adca0a2e824a8dafa906bf42
    [pos:tests/fixtures/monochrome.avif]=458ee397ada68eba9c90e4b7be795e5e5bc32f6f430487d4aecbf03831a18022
    [pos:tests/fixtures/kimono_rotate90.avif]=efcc7c58f0e80849c9b608d04719fe2b5c05d22349bee554d9501c45db235a5c
    [pos:tests/fixtures/fuzz/y_plane_divergence_match.avif]=3a9b88a38126b58fcb8778dd9dc13b290abbcfeb8a75d979971b7fa0894463ec
    [pos:tests/fixtures/fuzz/y_plane_roundtrip_avif1.avif]=e5ebc9d14b9a73377954928bd88a598ec42fb8913fdd12085f735a3f0aa2cdc8
    [pos:tests/fixtures/fuzz/y_plane_roundtrip_avif2.avif]=e621abf7fd5963ae4691389eb97719d38963fc74f970f812f7fd179e045096ae
  )

  check() {
    # check <test> <jsonfile> <expected-outcome> [extra predicate name]
    local name=$1 file=$2 want=$3 extra=${4:-}
    local got; got=$(field "$file" outcome)
    local ok=1
    [[ $got == "$want" ]] || ok=0
    if [[ -n $extra ]]; then
      case "$extra" in
        no-promotion) [[ "$(field "$file" final_files)" == "0" ]] || ok=0 ;;
        no-child) [[ "$(field "$file" child_started)" == "0" ]] || ok=0 ;;
        enforced) [[ "$(field "$file" enforceable)" == "1" && "$(field "$file" raise_blocked)" == "1" ]] || ok=0 ;;
        *) ok=0 ;;
      esac
    fi
    if (( ok == 1 )); then
      PASSED=$(( PASSED + 1 ))
      echo "PASS ${name} outcome=${got} extra=${extra:-none}"
    else
      FAILED=$(( FAILED + 1 ))
      echo "FAIL ${name} outcome=${got} want=${want} extra=${extra:-none}"
    fi
    cat "$file" >> "$jsonl"
  }

  rundir() {
    local d="$root/$1"
    mkdir -p "$d/stage" "$d/final"
    printf '%s' "$d"
  }

  local d="" got_sha="" red_file="pos_tests_fixtures_red_avif.planes.bin"
  local red_sha="${KNOWN[pos:tests/fixtures/red.avif]}"

  # ---- T01: the limits exist before the workload runs, and the child sees them ----
  d=$(rundir T01-control-limits)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T01.json" --test T01-control-limits \
    -- "$bin" --report-limits --out "$d/stage" >/dev/null 2>&1
  check T01-control-limits "$root/T01.json" OK enforced
  if [[ -f "$d/final/limits.json" ]] \
     && grep -q '"as_hard":"536870912"' "$d/final/limits.json" \
     && grep -q '"cpu_hard_s":"10"' "$d/final/limits.json"; then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T01-inner-readback as_hard_bytes=536870912 cpu_hard_sec=10 source=/proc/self/limits"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T01-inner-readback expected the child to observe 536870912 bytes and 10 s"
    if [[ -f "$d/final/limits.json" ]]; then cat "$d/final/limits.json"; fi
  fi

  # ---- T02: a real decode inside the normal isolation reproduces its digest ----
  d=$(rundir T02-control-decode)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T02.json" --test T02-control-decode \
    --case-file "$red_file" --expect-sha256 "$red_sha" \
    -- "$bin" --case pos:tests/fixtures/red.avif --pkg "$pkg" --out "$d/stage" --json "$d/stage/case.json" \
    >/dev/null 2>&1
  check T02-control-decode "$root/T02.json" OK enforced
  got_sha=""
  if [[ -f "$d/final/$red_file" ]]; then
    got_sha=$(sha256sum "$d/final/$red_file" | cut -d' ' -f1)
  fi
  if [[ $got_sha == "$red_sha" ]]; then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T02-promoted-digest sha256=${got_sha}"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T02-promoted-digest got=${got_sha:-none} want=${red_sha}"
  fi

  # ---- T03: 64 MiB cap cannot accommodate a 128 MiB touch ----
  d=$(rundir T03-memory-probe)
  bash "$me" run --mem-kib 65536 --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T03.json" --test T03-memory-probe \
    -- "$bin" --stress-allocate-bytes 134217728 --out "$d/stage" >/dev/null 2>&1
  check T03-memory-probe "$root/T03.json" REFUSED_MEMORY_LIMIT no-promotion
  if compgen -G "$d/stage/*" >/dev/null || compgen -G "$d/final/*" >/dev/null; then
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T03-no-residue staged or promoted files survived a memory denial"
  else
    PASSED=$(( PASSED + 1 ))
    echo "PASS T03-no-residue no staged or promoted output after memory denial"
  fi

  # ---- T04: 2 s CPU cap stops a 20 s busy loop ----
  d=$(rundir T04-cpu-probe)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec 2 --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T04.json" --test T04-cpu-probe \
    -- "$bin" --stress-spin-ms 20000 >/dev/null 2>&1
  check T04-cpu-probe "$root/T04.json" REFUSED_CPU_LIMIT no-promotion
  if awk -v c="$(field "$root/T04.json" child_cpu_sec)" 'BEGIN{exit !(c+0 >= 1.5 && c+0 < 10)}'; then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T04-cpu-accounting child_cpu_sec=$(field "$root/T04.json" child_cpu_sec) cap_sec=2"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T04-cpu-accounting child_cpu_sec=$(field "$root/T04.json" child_cpu_sec)"
  fi

  # ---- T05: 2 s wall cap stops a 20 s sleep ----
  d=$(rundir T05-wall-probe)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms 2000 \
    --stage "$d/stage" --final "$d/final" --json "$root/T05.json" --test T05-wall-probe \
    -- "$bin" --stress-sleep-ms 20000 >/dev/null 2>&1
  check T05-wall-probe "$root/T05.json" REFUSED_WALL_TIMEOUT no-promotion
  if awk -v w="$(field "$root/T05.json" wall_ms_observed)" 'BEGIN{exit !(w+0 >= 1900 && w+0 < 12000)}'; then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T05-wall-timing wall_ms_observed=$(field "$root/T05.json" wall_ms_observed) budget_ms=2000"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T05-wall-timing wall_ms_observed=$(field "$root/T05.json" wall_ms_observed)"
  fi

  # ---- T06: a descendant cannot outlive the contained group ----
  d=$(rundir T06-descendant)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms 2000 \
    --stage "$d/stage" --final "$d/final" --json "$root/T06.json" --test T06-descendant \
    -- "$bin" --stress-spawn-descendant 20000 --out "$d/stage" >/dev/null 2>&1
  check T06-descendant "$root/T06.json" REFUSED_WALL_TIMEOUT no-promotion
  local survivors=0
  survivors=$(pgrep -fc -- '--stress-sleep-ms 20000' 2>/dev/null)
  if [[ -z $survivors ]]; then survivors=0; fi
  if (( survivors == 0 )); then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T06-descendant-containment surviving_descendants=0"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T06-descendant-containment surviving_descendants=${survivors}"
  fi

  # ---- T07: the 20 MiB input ceiling is refused before any child exists ----
  d=$(rundir T07-input-boundary)
  local big="$d/oversize.bin"
  truncate -s 20971521 "$big"
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T07.json" --test T07-input-boundary \
    --input "$big" -- "$bin" --case neg:empty --out "$d/stage" --json "$d/stage/case.json" \
    >/dev/null 2>&1
  check T07-input-boundary "$root/T07.json" REFUSED_INPUT_TOO_LARGE no-child

  # ---- T08: a post-decode digest mismatch promotes nothing ----
  d=$(rundir T08-digest-mismatch)
  local wrong="ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T08.json" --test T08-digest-mismatch \
    --case-file "$red_file" --expect-sha256 "$wrong" \
    -- "$bin" --case pos:tests/fixtures/red.avif --pkg "$pkg" --out "$d/stage" --json "$d/stage/case.json" \
    >/dev/null 2>&1
  check T08-digest-mismatch "$root/T08.json" REFUSED_DIGEST_MISMATCH no-promotion

  # ---- T09: a workload that cannot be created is a typed refusal ----
  d=$(rundir T09-child-creation)
  bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T09.json" --test T09-child-creation \
    -- "$d/no-such-workload-xyz" >/dev/null 2>&1
  check T09-child-creation "$root/T09.json" REFUSED_CHILD_CREATION no-promotion

  # ---- T10: an unusable containment configuration refuses to run anything ----
  d=$(rundir T10-container-unavailable)
  bash "$me" run --mem-kib 0 --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stage" --final "$d/final" --json "$root/T10.json" --test T10-container-unavailable \
    -- "$bin" --report-limits --out "$d/stage" >/dev/null 2>&1
  check T10-container-unavailable "$root/T10.json" REFUSED_CONTAINER_UNAVAILABLE no-child

  # ---- T11: the whole accepted corpus, decoded inside the normal isolation ----
  local id safe sup st n_ok=0 n_bad=0
  while IFS= read -r id; do
    [[ -n $id ]] || continue
    safe=$(printf '%s' "$id" | tr -c 'A-Za-z0-9' '_' | sed 's/_*$//')
    d=$(rundir "T11-corpus/$safe")
    if [[ -n ${KNOWN[$id]:-} ]]; then
      bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
        --stage "$d/stage" --final "$d/final" --json "$root/T11-$safe.json" --test "T11-corpus:$id" \
        --case-file "${safe}.planes.bin" --expect-sha256 "${KNOWN[$id]}" \
        -- "$bin" --case "$id" --pkg "$pkg" --out "$d/stage" --json "$d/stage/case.json" \
        >/dev/null 2>&1
    else
      bash "$me" run --mem-kib $NORMAL_MEM --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
        --stage "$d/stage" --final "$d/final" --json "$root/T11-$safe.json" --test "T11-corpus:$id" \
        -- "$bin" --case "$id" --pkg "$pkg" --out "$d/stage" --json "$d/stage/case.json" \
        >/dev/null 2>&1
    fi
    sup=$(field "$root/T11-$safe.json" outcome)
    st=""
    if [[ -f "$d/final/case.json" ]]; then st=$(field "$d/final/case.json" status); fi
    if [[ $sup == "OK" && $st == "PASS" ]]; then
      n_ok=$(( n_ok + 1 ))
    else
      n_bad=$(( n_bad + 1 ))
      echo "FAIL T11-corpus case=${id} supervisor=${sup:-none} case_status=${st:-none}"
    fi
  done < <("$bin" --list --pkg "$pkg")
  if (( n_bad == 0 && n_ok == 25 )); then
    PASSED=$(( PASSED + 1 ))
    echo "PASS T11-corpus cases=${n_ok} digests=12/12 refusals=13/13"
  else
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T11-corpus ok=${n_ok} bad=${n_bad} expected 25"
  fi

  # ---- T12: two simultaneous contained workloads each stay inside their policy ----
  d=$(rundir T12-parallel)
  bash "$me" run --mem-kib 65536 --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stageA" --final "$d/finalA" --json "$root/T12a.json" --test T12-parallel-a \
    -- "$bin" --stress-allocate-bytes 134217728 --out "$d/stageA" >/dev/null 2>&1 &
  local pa=$!
  bash "$me" run --mem-kib 65536 --cpu-sec $NORMAL_CPU --wall-ms $NORMAL_WALL \
    --stage "$d/stageB" --final "$d/finalB" --json "$root/T12b.json" --test T12-parallel-b \
    -- "$bin" --stress-allocate-bytes 134217728 --out "$d/stageB" >/dev/null 2>&1 &
  local pb=$!
  wait "$pa"
  wait "$pb"
  check T12-parallel-a "$root/T12a.json" REFUSED_MEMORY_LIMIT no-promotion
  check T12-parallel-b "$root/T12b.json" REFUSED_MEMORY_LIMIT no-promotion
  if compgen -G "$d/finalA/*" >/dev/null || compgen -G "$d/finalB/*" >/dev/null; then
    FAILED=$(( FAILED + 1 ))
    echo "FAIL T12-parallel-residue a promoted file escaped a parallel denial"
  else
    PASSED=$(( PASSED + 1 ))
    echo "PASS T12-parallel-residue both parallel workloads refused with zero promotion"
  fi

  echo "suite-totals passed=${PASSED} failed=${FAILED}"
  if (( FAILED > 0 )); then
    return 1
  fi
  return 0
}


run_mode() {
  local stage="" final="" jsonf="" input="" expect="" casefile="" testid="run"
  local -a cmd=()
  while (( $# > 0 )); do
    case "$1" in
      --mem-kib) MEM_KIB=$2; shift 2 ;;
      --cpu-sec) CPU_SEC=$2; shift 2 ;;
      --wall-ms) WALL_MS=$2; shift 2 ;;
      --stage) stage=$2; shift 2 ;;
      --final) final=$2; shift 2 ;;
      --json) jsonf=$2; shift 2 ;;
      --input) input=$2; shift 2 ;;
      --expect-sha256) expect=$2; shift 2 ;;
      --case-file) casefile=$2; shift 2 ;;
      --test) testid=$2; shift 2 ;;
      --) shift; cmd=("$@"); break ;;
      *) usage ;;
    esac
  done
  [[ ${#cmd[@]} -gt 0 ]] || usage
  [[ -n $jsonf ]] || usage
  [[ -n $stage ]] || stage="$(dirname "$jsonf")/stage"
  [[ -n $final ]] || final="$(dirname "$jsonf")/final"
  : > "$jsonf"

  local outcome="" note="" child_started=0 timed_out=0 exit_code=-1 sig=0
  local enforceable=0 raise_blocked=0 started_ms=0 wall_ms_observed=0 child_cpu=0.000
  local stage_files=0 promoted=0 discarded=0 final_files=0
  local actual_sha="" input_bytes="null" sz=0

  # (1) Configuration validity: a supervisor that cannot understand its own caps
  # must refuse rather than run unconfined.
  if ! [[ $MEM_KIB =~ ^[0-9]+$ ]] || ! [[ $CPU_SEC =~ ^[0-9]+$ ]] || ! [[ $WALL_MS =~ ^[0-9]+$ ]] \
     || (( MEM_KIB <= 0 )) || (( CPU_SEC <= 0 )) || (( WALL_MS <= 0 )); then
    outcome="REFUSED_CONTAINER_UNAVAILABLE"
    note="invalid limit configuration"
  fi

  # (2) Caller-side input precondition, evaluated before any child exists.
  if [[ -z $outcome && -n $input ]]; then
    if [[ ! -f $input ]]; then
      outcome="REFUSED_INPUT_MISSING"
      note="input file not found"
    else
      sz=$(stat -c %s "$input")
      input_bytes=$sz
      if (( sz > MAX_INPUT_BYTES )); then
        outcome="REFUSED_INPUT_TOO_LARGE"
        note="input_bytes=${sz} limit_bytes=${MAX_INPUT_BYTES}"
      fi
    fi
  fi

  # (3) Prove the kernel accepts these limits and will not let them be raised again,
  # before the workload is created. Failure is a typed refusal, never an unconfined run.
  if [[ -z $outcome ]]; then
    if limits_installable "$MEM_KIB" "$CPU_SEC"; then
      enforceable=1
      raise_blocked=1
    else
      outcome="REFUSED_CONTAINER_UNAVAILABLE"
      note="kernel limits could not be installed or remained raisable"
    fi
  fi

  # (4) Contained execution: limits are applied inside the new session before exec,
  # so no instruction of the workload can run without them.
  if [[ -z $outcome ]]; then
    mkdir -p "$stage" "$final"
    find "$stage" -mindepth 1 -delete 2>/dev/null
    export PROBE_MEM_KIB=$MEM_KIB
    export PROBE_CPU_SEC=$CPU_SEC
    started_ms=$(now_ms)
    setsid bash -c '
      ulimit -Sv "$PROBE_MEM_KIB" 2>/dev/null || exit 90
      ulimit -Hv "$PROBE_MEM_KIB" 2>/dev/null || exit 90
      ulimit -St "$PROBE_CPU_SEC" 2>/dev/null || exit 90
      ulimit -Ht "$PROBE_CPU_SEC" 2>/dev/null || exit 90
      exec "$@"
    ' bash "${cmd[@]}" &
    local pid=$!
    child_started=1
    while kill -0 "$pid" 2>/dev/null; do
      if (( $(now_ms) - started_ms >= WALL_MS )); then
        timed_out=1
        kill -TERM -"$pid" 2>/dev/null
        kill -TERM "$pid" 2>/dev/null
        sleep 0.2
        kill -KILL -"$pid" 2>/dev/null
        kill -KILL "$pid" 2>/dev/null
        break
      fi
      sleep 0.05
    done
    wait "$pid" 2>/dev/null
    exit_code=$?
    # Sweep any surviving members of the contained process group: nothing may outlive
    # the supervisor, on the normal-exit path as well as the timeout path.
    kill -KILL -"$pid" 2>/dev/null
    wall_ms_observed=$(( $(now_ms) - started_ms ))
    local tfile="${TMPDIR:-/tmp}/wo070-times-$$-${RANDOM}"
    times > "$tfile"
    child_cpu=$(child_cpu_seconds "$tfile")
    rm -f "$tfile"
    if (( exit_code > 128 )); then sig=$(( exit_code - 128 )); fi
  fi

  # (5) Classify the termination reason from the raw facts, never from a guess.
  if [[ -z $outcome ]]; then
    if (( timed_out == 1 )); then
      outcome="REFUSED_WALL_TIMEOUT"
      note="wall_ms=${wall_ms_observed} budget_ms=${WALL_MS}"
    elif (( exit_code == 90 )); then
      outcome="REFUSED_CONTAINER_UNAVAILABLE"
      note="limit application failed inside the contained shell"
    elif (( exit_code == 127 )); then
      outcome="REFUSED_CHILD_CREATION"
      note="contained program could not be executed"
    elif (( exit_code == 0 )); then
      outcome="OK"
    elif (( sig == 24 || sig == 9 )); then
      outcome="REFUSED_CPU_LIMIT"
      note="signal=${sig} child_cpu_sec=${child_cpu} cap_sec=${CPU_SEC}"
    elif (( sig == 6 || sig == 11 )); then
      outcome="REFUSED_MEMORY_LIMIT"
      note="signal=${sig} address-space denial aborts the workload"
    else
      outcome="REFUSED_ABNORMAL_EXIT"
      note="exit_code=${exit_code} signal=${sig}"
    fi
  fi

  # (6) Output discipline: verify the decoded-plane digest with the host tool, then
  # promote; on any refusal discard every staged byte and promote nothing.
  stage_files=$(find "$stage" -maxdepth 1 -type f 2>/dev/null | wc -l | tr -d ' ')
  if [[ $outcome == "OK" && -n $casefile ]]; then
    if [[ ! -f "$stage/$casefile" ]]; then
      outcome="REFUSED_MISSING_OUTPUT"
      note="expected staged file absent: ${casefile}"
    else
      actual_sha=$(sha256sum "$stage/$casefile" | cut -d' ' -f1)
      if [[ -n $expect && $actual_sha != "$expect" ]]; then
        outcome="REFUSED_DIGEST_MISMATCH"
        note="decoded_sha256=${actual_sha} expected=${expect}"
      fi
    fi
  fi

  if [[ $outcome == "OK" ]]; then
    if [[ -d $stage ]]; then
      for f in "$stage"/*; do
        [[ -e $f ]] || continue
        mv -f "$f" "$final/"
        promoted=$(( promoted + 1 ))
      done
    fi
  else
    if [[ -d $stage ]]; then
      discarded=$stage_files
      find "$stage" -mindepth 1 -delete 2>/dev/null
    fi
  fi
  if [[ -d $final ]]; then final_files=$(find "$final" -maxdepth 1 -type f 2>/dev/null | wc -l | tr -d ' '); fi

  local ok=false
  if [[ $outcome == "OK" ]]; then ok=true; fi
  emit_record "$jsonf" \
    "\"test\":\"$(json_escape "$testid")\",\"mode\":\"run\",\"platform\":\"linux\",\"outcome\":\"${outcome}\",\"ok\":${ok},\"note\":\"$(json_escape "$note")\",\"limits\":{\"as_kib\":${MEM_KIB},\"as_bytes\":$(( MEM_KIB * 1024 )),\"cpu_sec\":${CPU_SEC},\"wall_ms\":${WALL_MS}},\"enforceable\":${enforceable},\"raise_blocked\":${raise_blocked},\"child_started\":${child_started},\"exit_code\":${exit_code},\"signal\":${sig},\"timed_out\":${timed_out},\"wall_ms_observed\":${wall_ms_observed},\"child_cpu_sec\":${child_cpu},\"input_bytes\":${input_bytes},\"stage_files\":${stage_files},\"promoted\":${promoted},\"discarded\":${discarded},\"final_files\":${final_files},\"expected_sha256\":\"$(json_escape "$expect")\",\"actual_sha256\":\"$(json_escape "$actual_sha")\",\"cmd_basename\":\"$(json_escape "$(basename "${cmd[0]}")")\""
  return 0
}

case "${1:-}" in
  run) shift; run_mode "$@" ;;
  suite) shift; suite_mode "$@" ;;
  *) usage ;;
esac
