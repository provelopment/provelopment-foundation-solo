# Windows containment launcher and test driver for the standalone AVIF proof.
#
# NON-PRODUCT EVIDENCE for `SOLO-WO-070 R01`. Nothing here is product code and
# nothing here approves a decoder.
#
# What it does: creates a Windows Job Object, installs hard per-process CPU time,
# per-process commit and job commit limits plus kill-on-job-close; creates the
# workload SUSPENDED, assigns it to the limited job, VERIFIES assignment and the
# effective limits, and only then resumes its first thread. A finite parent deadline
# terminates the whole job, so no descendant can outlive containment.
#
# Every native call is checked. If the job cannot be created, configured, verified,
# assigned or resumed, the run refuses without ever resuming the workload — there is
# deliberately no unsuspended Start-Process fallback.
param(
  [Parameter(ValueFromRemainingArguments = $true)]
  [string[]]$Arguments
)

$ErrorActionPreference = 'Stop'
$MaxInputBytes = 20971520
$DefaultMemBytes = 536870912
$DefaultCpuSec = 10
$DefaultWallMs = 15000
# Re-invocations (the parallel probes) must use the same PowerShell host that is
# running this script, so the compiled C# interop surface is identical.
$hostExe = [System.Diagnostics.Process]::GetCurrentProcess().MainModule.FileName

$NativeSource = @'
using System;
using System.Runtime.InteropServices;

public static class SoloJobRunner
{
    [StructLayout(LayoutKind.Sequential)]
    public struct JOBOBJECT_BASIC_LIMIT_INFORMATION
    {
        public long PerProcessUserTimeLimit;
        public long PerJobUserTimeLimit;
        public uint LimitFlags;
        public UIntPtr MinimumWorkingSetSize;
        public UIntPtr MaximumWorkingSetSize;
        public uint ActiveProcessLimit;
        public UIntPtr Affinity;
        public uint PriorityClass;
        public uint SchedulingClass;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct IO_COUNTERS
    {
        public ulong ReadOperationCount;
        public ulong WriteOperationCount;
        public ulong OtherOperationCount;
        public ulong ReadTransferCount;
        public ulong WriteTransferCount;
        public ulong OtherTransferCount;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION
    {
        public JOBOBJECT_BASIC_LIMIT_INFORMATION BasicLimitInformation;
        public IO_COUNTERS IoInfo;
        public UIntPtr ProcessMemoryLimit;
        public UIntPtr JobMemoryLimit;
        public UIntPtr PeakProcessMemoryUsed;
        public UIntPtr PeakJobMemoryUsed;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION
    {
        public long TotalUserTime;
        public long TotalKernelTime;
        public long ThisPeriodTotalUserTime;
        public long ThisPeriodTotalKernelTime;
        public uint TotalPageFaultCount;
        public uint TotalProcesses;
        public uint ActiveProcesses;
        public uint TotalTerminatedProcesses;
    }

    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)]
    public struct STARTUPINFO
    {
        public int cb;
        public string lpReserved;
        public string lpDesktop;
        public string lpTitle;
        public int dwX;
        public int dwY;
        public int dwXSize;
        public int dwYSize;
        public int dwXCountChars;
        public int dwYCountChars;
        public int dwFillAttribute;
        public int dwFlags;
        public short wShowWindow;
        public short cbReserved2;
        public IntPtr lpReserved2;
        public IntPtr hStdInput;
        public IntPtr hStdOutput;
        public IntPtr hStdError;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct PROCESS_INFORMATION
    {
        public IntPtr hProcess;
        public IntPtr hThread;
        public int dwProcessId;
        public int dwThreadId;
    }

    const uint JOB_OBJECT_LIMIT_PROCESS_TIME = 0x00000002;
    const uint JOB_OBJECT_LIMIT_PROCESS_MEMORY = 0x00000100;
    const uint JOB_OBJECT_LIMIT_JOB_MEMORY = 0x00000200;
    const uint JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE = 0x00002000;
    const int JobObjectExtendedLimitInformation = 9;
    const uint CREATE_SUSPENDED = 0x00000004;
    const uint CREATE_NO_WINDOW = 0x08000000;
    const uint WAIT_TIMEOUT = 258;

    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    static extern IntPtr CreateJobObjectW(IntPtr lpJobAttributes, string lpName);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool SetInformationJobObject(IntPtr hJob, int infoClass, IntPtr lpInfo, uint cbInfo);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool QueryInformationJobObject(IntPtr hJob, int infoClass, IntPtr lpInfo, uint cbInfo, out uint lpReturnLength);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool AssignProcessToJobObject(IntPtr hJob, IntPtr hProcess);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool IsProcessInJob(IntPtr hProcess, IntPtr hJob, out bool result);

    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    static extern bool CreateProcessW(string lpApplicationName, string lpCommandLine, IntPtr lpProcessAttributes,
        IntPtr lpThreadAttributes, bool bInheritHandles, uint dwCreationFlags, IntPtr lpEnvironment,
        string lpCurrentDirectory, ref STARTUPINFO lpStartupInfo, out PROCESS_INFORMATION lpProcessInformation);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern uint ResumeThread(IntPtr hThread);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern uint WaitForSingleObject(IntPtr hHandle, uint dwMilliseconds);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool TerminateJobObject(IntPtr hJob, uint uExitCode);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool GetExitCodeProcess(IntPtr hProcess, out uint lpExitCode);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool CloseHandle(IntPtr hObject);

    public class Result
    {
        public bool SetupOk;
        public string Stage = "none";
        public int ErrorCode;
        public bool ChildCreated;
        public bool Assigned;
        public bool Verified;
        public bool Resumed;
        public bool Terminated;
        public uint ExitCode;
        public long WallMs;
        public long PeakProcessBytes;
        public long PeakJobBytes;
        public long PeakCpuLimit100ns;
        public uint LimitFlags;
        public long ProcessMemoryLimit;
        public long JobMemoryLimit;
        public long TotalUserTime100ns;
        public long TotalKernelTime100ns;
        public uint TotalProcesses;
        public uint ActiveProcesses;
        public uint TotalTerminatedProcesses;
    }

    static Result Fail(Result r, string stage, int code)
    {
        r.Stage = stage;
        r.ErrorCode = code;
        r.SetupOk = false;
        return r;
    }

    public static Result Run(string application, string commandLine, long memoryBytes, long cpuSeconds, uint wallMs)
    {
        Result r = new Result();
        IntPtr job = IntPtr.Zero;
        PROCESS_INFORMATION pi = new PROCESS_INFORMATION();
        bool haveProcess = false;
        try
        {
            job = CreateJobObjectW(IntPtr.Zero, null);
            if (job == IntPtr.Zero) return Fail(r, "create-job", Marshal.GetLastWin32Error());

            JOBOBJECT_EXTENDED_LIMIT_INFORMATION info = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();
            info.BasicLimitInformation.LimitFlags =
                JOB_OBJECT_LIMIT_PROCESS_TIME | JOB_OBJECT_LIMIT_PROCESS_MEMORY |
                JOB_OBJECT_LIMIT_JOB_MEMORY | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            info.BasicLimitInformation.PerProcessUserTimeLimit = cpuSeconds * 10000000L;
            info.ProcessMemoryLimit = (UIntPtr)(ulong)memoryBytes;
            info.JobMemoryLimit = (UIntPtr)(ulong)memoryBytes;

            int size = Marshal.SizeOf(typeof(JOBOBJECT_EXTENDED_LIMIT_INFORMATION));
            IntPtr buffer = Marshal.AllocHGlobal(size);
            try
            {
                Marshal.StructureToPtr(info, buffer, false);
                if (!SetInformationJobObject(job, JobObjectExtendedLimitInformation, buffer, (uint)size))
                    return Fail(r, "set-info", Marshal.GetLastWin32Error());
            }
            finally { Marshal.FreeHGlobal(buffer); }

            JOBOBJECT_EXTENDED_LIMIT_INFORMATION effective = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();
            uint returned = 0;
            buffer = Marshal.AllocHGlobal(size);
            try
            {
                Marshal.StructureToPtr(effective, buffer, false);
                if (!QueryInformationJobObject(job, JobObjectExtendedLimitInformation, buffer, (uint)size, out returned))
                    return Fail(r, "query-info", Marshal.GetLastWin32Error());
                effective = (JOBOBJECT_EXTENDED_LIMIT_INFORMATION)Marshal.PtrToStructure(buffer, typeof(JOBOBJECT_EXTENDED_LIMIT_INFORMATION));
            }
            finally { Marshal.FreeHGlobal(buffer); }

            r.LimitFlags = effective.BasicLimitInformation.LimitFlags;
            r.PeakCpuLimit100ns = effective.BasicLimitInformation.PerProcessUserTimeLimit;
            r.ProcessMemoryLimit = (long)(ulong)effective.ProcessMemoryLimit;
            r.JobMemoryLimit = (long)(ulong)effective.JobMemoryLimit;
            bool flagsOk = (r.LimitFlags & JOB_OBJECT_LIMIT_PROCESS_TIME) != 0
                        && (r.LimitFlags & JOB_OBJECT_LIMIT_PROCESS_MEMORY) != 0
                        && (r.LimitFlags & JOB_OBJECT_LIMIT_JOB_MEMORY) != 0
                        && (r.LimitFlags & JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE) != 0;
            if (!flagsOk) return Fail(r, "verify-flags", 0);
            if (r.PeakCpuLimit100ns != cpuSeconds * 10000000L) return Fail(r, "verify-cpu", 0);
            if (r.ProcessMemoryLimit != memoryBytes || r.JobMemoryLimit != memoryBytes) return Fail(r, "verify-memory", 0);
            r.Verified = true;

            STARTUPINFO si = new STARTUPINFO();
            si.cb = Marshal.SizeOf(typeof(STARTUPINFO));
            if (!CreateProcessW(application, commandLine, IntPtr.Zero, IntPtr.Zero, false,
                    CREATE_SUSPENDED | CREATE_NO_WINDOW, IntPtr.Zero, null, ref si, out pi))
                return Fail(r, "create-process", Marshal.GetLastWin32Error());
            haveProcess = true;
            r.ChildCreated = true;

            if (!AssignProcessToJobObject(job, pi.hProcess))
            {
                TerminateJobObject(job, 1);
                return Fail(r, "assign", Marshal.GetLastWin32Error());
            }
            r.Assigned = true;

            bool inJob;
            if (!IsProcessInJob(pi.hProcess, job, out inJob))
            {
                TerminateJobObject(job, 1);
                return Fail(r, "is-in-job", Marshal.GetLastWin32Error());
            }
            if (!inJob)
            {
                TerminateJobObject(job, 1);
                return Fail(r, "not-in-job", 0);
            }

            if (ResumeThread(pi.hThread) == 0xFFFFFFFF)
            {
                TerminateJobObject(job, 1);
                return Fail(r, "resume", Marshal.GetLastWin32Error());
            }
            r.Resumed = true;
            r.SetupOk = true;
            r.Stage = "running";

            System.Diagnostics.Stopwatch clock = System.Diagnostics.Stopwatch.StartNew();
            uint wait = WaitForSingleObject(pi.hProcess, wallMs);
            if (wait == WAIT_TIMEOUT)
            {
                r.Terminated = TerminateJobObject(job, 1);
                WaitForSingleObject(pi.hProcess, 10000);
            }
            r.WallMs = clock.ElapsedMilliseconds;

            uint exitCode;
            if (!GetExitCodeProcess(pi.hProcess, out exitCode)) return Fail(r, "get-exit", Marshal.GetLastWin32Error());
            r.ExitCode = exitCode;

            JOBOBJECT_EXTENDED_LIMIT_INFORMATION after = new JOBOBJECT_EXTENDED_LIMIT_INFORMATION();
            buffer = Marshal.AllocHGlobal(size);
            try
            {
                Marshal.StructureToPtr(after, buffer, false);
                if (QueryInformationJobObject(job, JobObjectExtendedLimitInformation, buffer, (uint)size, out returned))
                {
                    after = (JOBOBJECT_EXTENDED_LIMIT_INFORMATION)Marshal.PtrToStructure(buffer, typeof(JOBOBJECT_EXTENDED_LIMIT_INFORMATION));
                    r.PeakProcessBytes = (long)(ulong)after.PeakProcessMemoryUsed;
                    r.PeakJobBytes = (long)(ulong)after.PeakJobMemoryUsed;
                }
            }
            finally { Marshal.FreeHGlobal(buffer); }

            JOBOBJECT_BASIC_ACCOUNTING_INFORMATION accounting = new JOBOBJECT_BASIC_ACCOUNTING_INFORMATION();
            int accSize = Marshal.SizeOf(typeof(JOBOBJECT_BASIC_ACCOUNTING_INFORMATION));
            IntPtr accBuffer = Marshal.AllocHGlobal(accSize);
            try
            {
                Marshal.StructureToPtr(accounting, accBuffer, false);
                uint accReturned;
                if (QueryInformationJobObject(job, 1, accBuffer, (uint)accSize, out accReturned))
                {
                    accounting = (JOBOBJECT_BASIC_ACCOUNTING_INFORMATION)Marshal.PtrToStructure(accBuffer, typeof(JOBOBJECT_BASIC_ACCOUNTING_INFORMATION));
                    r.TotalUserTime100ns = accounting.TotalUserTime;
                    r.TotalKernelTime100ns = accounting.TotalKernelTime;
                    r.TotalProcesses = accounting.TotalProcesses;
                    r.ActiveProcesses = accounting.ActiveProcesses;
                    r.TotalTerminatedProcesses = accounting.TotalTerminatedProcesses;
                }
            }
            finally { Marshal.FreeHGlobal(accBuffer); }

            r.Stage = "completed";
            return r;
        }
        finally
        {
            if (haveProcess)
            {
                if (pi.hThread != IntPtr.Zero) CloseHandle(pi.hThread);
                if (pi.hProcess != IntPtr.Zero) CloseHandle(pi.hProcess);
            }
            if (job != IntPtr.Zero) CloseHandle(job);
        }
    }
}
'@

if (-not ('SoloJobRunner' -as [type])) {
  Add-Type -TypeDefinition $NativeSource -Language CSharp
}

function Invoke-Run {
  $a = $Arguments
  $stage = ''
  $final = ''
  $json = ''
  $inputFile = ''
  $expect = ''
  $caseFile = ''
  $testId = 'run'
  $mem = $DefaultMemBytes
  $cpu = $DefaultCpuSec
  $wall = $DefaultWallMs
  $cmd = @()
  $i = 0
  while ($i -lt $a.Count) {
    $tok = $a[$i]
    if ($tok -eq '--mem-bytes') { $mem = [long]$a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--cpu-sec') { $cpu = [long]$a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--wall-ms') { $wall = [long]$a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--stage') { $stage = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--final') { $final = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--json') { $json = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--input') { $inputFile = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--expect-sha256') { $expect = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--case-file') { $caseFile = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--test') { $testId = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--' -or $tok -eq '::') { $cmd = @($a[($i + 1)..($a.Count - 1)]); $i = $a.Count }
    else { Write-Error "unknown argument: $tok"; exit 2 }
  }
  if ($cmd.Count -lt 1 -or -not $json) { Write-Error 'usage: run ... -- CMD [ARGS]'; exit 2 }
  if (-not $stage) { $stage = Join-Path (Split-Path -Parent $json) 'stage' }
  if (-not $final) { $final = Join-Path (Split-Path -Parent $json) 'final' }
  if (Test-Path -LiteralPath $json) { Remove-Item -LiteralPath $json -Force }

  $outcome = ''
  $note = ''
  $childStarted = 0
  $timedOut = 0
  $exitCode = -1
  $inputBytes = 'null'
  $actualSha = ''
  $stageFiles = 0
  $promoted = 0
  $discarded = 0
  $finalFiles = 0
  $job = $null

  if ($mem -le 0 -or $cpu -le 0 -or $wall -le 0) {
    $outcome = 'REFUSED_CONTAINER_UNAVAILABLE'
    $note = 'invalid limit configuration'
  }

  if (-not $outcome -and $inputFile) {
    if (-not (Test-Path -LiteralPath $inputFile)) {
      $outcome = 'REFUSED_INPUT_MISSING'
      $note = 'input file not found'
    } else {
      $sz = (Get-Item -LiteralPath $inputFile).Length
      $inputBytes = $sz
      if ($sz -gt $MaxInputBytes) {
        $outcome = 'REFUSED_INPUT_TOO_LARGE'
        $note = "input_bytes=$sz limit_bytes=$MaxInputBytes"
      }
    }
  }

  if (-not $outcome) {
    New-Item -ItemType Directory -Force -Path $stage | Out-Null
    New-Item -ItemType Directory -Force -Path $final | Out-Null
    Get-ChildItem -LiteralPath $stage -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
    $exe = $cmd[0]
    $quoted = foreach ($part in $cmd) { if ($part -match '\s') { '"' + $part + '"' } else { $part } }
    $cmdline = $quoted -join ' '
    $job = [SoloJobRunner]::Run($exe, $cmdline, [long]$mem, [long]$cpu, [uint32]$wall)
    if (-not $job.SetupOk) {
      if ($job.Stage -eq 'create-process') {
        $outcome = 'REFUSED_CHILD_CREATION'
        $note = "create_process_error=$($job.ErrorCode)"
      } else {
        $outcome = 'REFUSED_CONTAINER_UNAVAILABLE'
        $note = "job_stage=$($job.Stage) error=$($job.ErrorCode)"
      }
      if ($job.ChildCreated) { $childStarted = 1 }
    } else {
      $childStarted = 1
      $exitCode = [long]$job.ExitCode
      if ($job.Terminated) {
        $outcome = 'REFUSED_WALL_TIMEOUT'
        $timedOut = 1
        $note = "wall_ms=$($job.WallMs) budget_ms=$wall"
      } elseif ($job.ExitCode -eq 0) {
        $outcome = 'OK'
      } elseif ([long]$job.ExitCode -eq 3221226505 -or [long]$job.ExitCode -eq 3221225495 -or
                [long]$job.ExitCode -eq 3221225725 -or [long]$job.ExitCode -eq 4294967295 -or
                [long]$job.ExitCode -eq 3) {
        # 3221226505 = 0xC0000409 (abort/fast-fail), 3221225495 = 0xC0000017 (no memory),
        # 3221225725 = 0xC00000FD (stack overflow), 4294967295 = 0xFFFFFFFF, 3 = abort().
        $outcome = 'REFUSED_MEMORY_LIMIT'
        $note = ("exit_code=0x{0:X8} peak_process_bytes={1} limit_bytes={2}" -f $job.ExitCode, $job.PeakProcessBytes, $mem)
      } elseif ($job.PeakProcessBytes -gt 0 -and $job.PeakProcessBytes -ge ([long]$mem * 0.9) -and $job.ExitCode -ne 0) {
        $outcome = 'REFUSED_MEMORY_LIMIT'
        $note = ("peak_process_bytes={0} reached the {1}-byte job ceiling; exit_code=0x{2:X8}" -f $job.PeakProcessBytes, $mem, $job.ExitCode)
      } elseif ($job.TotalUserTime100ns -ge ([long]$cpu * 9000000)) {
        $outcome = 'REFUSED_CPU_LIMIT'
        $note = ("job_user_time_100ns={0} cap_100ns={1}" -f $job.TotalUserTime100ns, ([long]$cpu * 10000000))
      } else {
        $outcome = 'REFUSED_ABNORMAL_EXIT'
        $note = ("exit_code=0x{0:X8} active_processes={1}" -f $job.ExitCode, $job.ActiveProcesses)
      }
    }
  }

  $stageFiles = @(Get-ChildItem -LiteralPath $stage -File -ErrorAction SilentlyContinue).Count
  if ($outcome -eq 'OK' -and $caseFile) {
    $staged = Join-Path $stage $caseFile
    if (-not (Test-Path -LiteralPath $staged)) {
      $outcome = 'REFUSED_MISSING_OUTPUT'
      $note = "expected staged file absent: $caseFile"
    } else {
      $actualSha = (Get-FileHash -LiteralPath $staged -Algorithm SHA256).Hash.ToLower()
      if ($expect -and $actualSha -ne $expect.ToLower()) {
        $outcome = 'REFUSED_DIGEST_MISMATCH'
        $note = "decoded_sha256=$actualSha expected=$($expect.ToLower())"
      }
    }
  }

  if ($outcome -eq 'OK') {
    foreach ($f in @(Get-ChildItem -LiteralPath $stage -File -ErrorAction SilentlyContinue)) {
      Move-Item -LiteralPath $f.FullName -Destination (Join-Path $final $f.Name) -Force
      $promoted++
    }
  } else {
    $discarded = $stageFiles
    Get-ChildItem -LiteralPath $stage -Force -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force -ErrorAction SilentlyContinue
  }
  $finalFiles = @(Get-ChildItem -LiteralPath $final -File -ErrorAction SilentlyContinue).Count

  $ok = if ($outcome -eq 'OK') { 'true' } else { 'false' }
  $flags = 0
  $cpuLimit = 0
  $procLimit = 0
  $jobLimit = 0
  $peakProc = 0
  $peakJob = 0
  $jobUser = 0
  $jobActive = 0
  $jobTotal = 0
  $jobStage = 'not-run'
  $jobError = 0
  if ($null -ne $job) {
    $flags = $job.LimitFlags
    $cpuLimit = $job.PeakCpuLimit100ns
    $procLimit = $job.ProcessMemoryLimit
    $jobLimit = $job.JobMemoryLimit
    $peakProc = $job.PeakProcessBytes
    $peakJob = $job.PeakJobBytes
    $jobUser = $job.TotalUserTime100ns
    $jobActive = $job.ActiveProcesses
    $jobTotal = $job.TotalProcesses
    $jobStage = $job.Stage
    $jobError = $job.ErrorCode
  }
  $verified = if ($null -ne $job -and $job.Verified) { 1 } else { 0 }
  $termStr = if ($null -ne $job -and $job.Terminated) { 'true' } else { 'false' }
  $assignedStr = if ($null -ne $job -and $job.Assigned) { 'true' } else { 'false' }
  $verifiedStr = if ($null -ne $job -and $job.Verified) { 'true' } else { 'false' }
  $noteEsc = ($note -replace '\\', '\\' -replace '"', '\"' -replace "`r", '' -replace "`n", ' ')
  $testEsc = ($testId -replace '\\', '\\' -replace '"', '\"')
  $exeName = Split-Path -Leaf $cmd[0]
  $record = '{"test":"' + $testEsc + '","mode":"run","platform":"windows","outcome":"' + $outcome +
    '","ok":' + $ok + ',"note":"' + $noteEsc + '","limits":{"mem_bytes":' + $mem + ',"cpu_sec":' + $cpu +
    ',"wall_ms":' + $wall + '},"enforceable":' + $verified + ',"raise_blocked":' + $verified +
    ',"child_started":' + $childStarted + ',"exit_code":' + $exitCode + ',"signal":0,"timed_out":' + $timedOut +
    ',"wall_ms_observed":' + $(if ($null -ne $job) { $job.WallMs } else { 0 }) +
    ',"child_cpu_sec":' + $(if ($jobUser -gt 0) { [math]::Round($jobUser / 10000000.0, 3) } else { 0 }) +
    ',"input_bytes":' + $inputBytes + ',"stage_files":' + $stageFiles + ',"promoted":' + $promoted +
    ',"discarded":' + $discarded + ',"final_files":' + $finalFiles +
    ',"expected_sha256":"' + $expect + '","actual_sha256":"' + $actualSha +
    '","cmd_basename":"' + $exeName + '"' +
    ',"job":{"stage":"' + $jobStage + '","error_code":' + $jobError + ',"limit_flags":' + $flags +
    ',"verified":' + $verifiedStr + ',"assigned":' + $assignedStr + ',"terminated":' + $termStr +
    ',"per_process_user_time_100ns":' + $cpuLimit + ',"process_memory_limit":' + $procLimit +
    ',"job_memory_limit":' + $jobLimit + ',"peak_process_bytes":' + $peakProc +
    ',"peak_job_bytes":' + $peakJob + ',"total_user_time_100ns":' + $jobUser +
    ',"total_processes":' + $jobTotal + ',"active_processes":' + $jobActive + '}}'
  Add-Content -LiteralPath $json -Value $record -Encoding utf8
  return 0
}

$KnownDigests = @{
  'pos:tests/fixtures/alpha_video.avif'                   = '3d26373d70bb0b2f0ced1ec923f1f51a592dd45e6e05706569e9994714e6b123'
  'pos:tests/fixtures/bbb_alpha.avif'                     = 'ad25ae289b08e19c58b27a60a538f82e2085876ee9aa265a922bdffe10844f6f'
  'pos:tests/fixtures/black420.avif'                      = 'd7928dbf7570fb114f49e426af9f36e70a9b4450b493cfbf6767826ab1cf63bb'
  'pos:tests/fixtures/gray32.avif'                        = 'ca707edbab56569fcbef9523c0a6661caf2de0f0b7e205169ba2a246a3a3f3d5'
  'pos:tests/fixtures/midgray.avif'                       = 'b4052e7408e5fa2bc3bdcd8aab8d1f85893825d36bb13b13f1112e6b7fce6d8a'
  'pos:tests/fixtures/red.avif'                           = '205691271feb77cc0f208f495bf466e2fb9a9733d816b2d2355f05eb6c15552c'
  'pos:tests/fixtures/white16.avif'                       = 'dec238c044dec03455ecb89858492569cbd64026adca0a2e824a8dafa906bf42'
  'pos:tests/fixtures/monochrome.avif'                    = '458ee397ada68eba9c90e4b7be795e5e5bc32f6f430487d4aecbf03831a18022'
  'pos:tests/fixtures/kimono_rotate90.avif'               = 'efcc7c58f0e80849c9b608d04719fe2b5c05d22349bee554d9501c45db235a5c'
  'pos:tests/fixtures/fuzz/y_plane_divergence_match.avif' = '3a9b88a38126b58fcb8778dd9dc13b290abbcfeb8a75d979971b7fa0894463ec'
  'pos:tests/fixtures/fuzz/y_plane_roundtrip_avif1.avif'  = 'e5ebc9d14b9a73377954928bd88a598ec42fb8913fdd12085f735a3f0aa2cdc8'
  'pos:tests/fixtures/fuzz/y_plane_roundtrip_avif2.avif'  = 'e621abf7fd5963ae4691389eb97719d38963fc74f970f812f7fd179e045096ae'
}

function Invoke-Supervisor {
  param([string[]]$SupArgs, [string]$RecordPath)
  & $PSCommandPath @SupArgs | Out-Null
  if (-not (Test-Path -LiteralPath $RecordPath)) { return $null }
  return ((Get-Content -LiteralPath $RecordPath -Raw).Trim() | ConvertFrom-Json)
}

function Invoke-Suite {
  $a = $Arguments
  $bin = ''
  $pkg = ''
  $root = ''
  $jsonl = ''
  $i = 0
  while ($i -lt $a.Count) {
    $tok = $a[$i]
    if ($tok -eq '--bin') { $bin = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--pkg') { $pkg = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--root') { $root = $a[$i + 1]; $i += 2 }
    elseif ($tok -eq '--jsonl') { $jsonl = $a[$i + 1]; $i += 2 }
    else { Write-Error "unknown argument: $tok"; exit 2 }
  }
  if (-not $bin -or -not $pkg -or -not $root -or -not $jsonl) { Write-Error 'usage: suite --bin --pkg --root --jsonl'; exit 2 }
  New-Item -ItemType Directory -Force -Path $root | Out-Null
  if (Test-Path -LiteralPath $jsonl) { Remove-Item -LiteralPath $jsonl -Force }
  $script:Passed = 0
  $script:Failed = 0
  $normalMem = 536870912
  $normalCpu = 10
  $normalWall = 15000
  $redFile = 'pos_tests_fixtures_red_avif.planes.bin'
  $redSha = $KnownDigests['pos:tests/fixtures/red.avif']

  function New-Dir([string]$name) {
    $d = Join-Path $root $name
    New-Item -ItemType Directory -Force -Path (Join-Path $d 'stage') | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $d 'final') | Out-Null
    return $d
  }

  function Check([string]$name, $rec, [string]$want, [string]$extra = '') {
    $ok = $true
    if ($null -eq $rec) { $ok = $false }
    elseif ($rec.outcome -ne $want) { $ok = $false }
    elseif ($extra -eq 'no-promotion' -and $rec.final_files -ne 0) { $ok = $false }
    elseif ($extra -eq 'no-child' -and $rec.child_started -ne 0) { $ok = $false }
    elseif ($extra -eq 'enforced' -and ($rec.enforceable -ne 1 -or $rec.job.limit_flags -eq 0)) { $ok = $false }
    if ($ok) {
      $script:Passed++
      Write-Output "PASS $name outcome=$($rec.outcome) extra=$extra"
    } else {
      $script:Failed++
      $got = if ($null -eq $rec) { 'no-record' } else { $rec.outcome }
      Write-Output "FAIL $name outcome=$got want=$want extra=$extra"
    }
    $p = Join-Path $root "$name.json"
    if (Test-Path -LiteralPath $p) {
      Add-Content -LiteralPath $jsonl -Value (Get-Content -LiteralPath $p -Raw).Trim() -Encoding utf8
    }
  }

  # T01 - the job object carries verified limits before the workload runs
  $d = New-Dir 'T01-control-limits'
  $r1 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T01-control-limits.json'),
    '--test', 'T01-control-limits', '--', $bin, '--report-limits', '--out', (Join-Path $d 'stage')) (Join-Path $root 'T01-control-limits.json')
  Check 'T01-control-limits' $r1 'OK' 'enforced'
  if ($null -ne $r1 -and $r1.job.per_process_user_time_100ns -eq 100000000 -and $r1.job.process_memory_limit -eq 536870912 -and
      $r1.job.job_memory_limit -eq 536870912 -and $r1.job.total_processes -ge 1) {
    $script:Passed++
    Write-Output ("PASS T01-job-limits cpu_100ns={0} process_memory={1} job_memory={2} total_processes={3} peak_job={4}" -f `
      $r1.job.per_process_user_time_100ns, $r1.job.process_memory_limit, $r1.job.job_memory_limit, $r1.job.total_processes, $r1.job.peak_job_bytes)
  } else {
    $script:Failed++
    Write-Output 'FAIL T01-job-limits queried limits or accounting did not match'
  }

  # T02 - a real decode inside the job reproduces its digest
  $d = New-Dir 'T02-control-decode'
  $r2 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T02-control-decode.json'),
    '--test', 'T02-control-decode', '--case-file', $redFile, '--expect-sha256', $redSha,
    '--', $bin, '--case', 'pos:tests/fixtures/red.avif', '--pkg', $pkg, '--out', (Join-Path $d 'stage'), '--json', (Join-Path $d 'stage/case.json')) (Join-Path $root 'T02-control-decode.json')
  Check 'T02-control-decode' $r2 'OK' 'enforced'
  $promotedRed = Join-Path $d "final/$redFile"
  if ((Test-Path -LiteralPath $promotedRed) -and ((Get-FileHash -LiteralPath $promotedRed -Algorithm SHA256).Hash.ToLower() -eq $redSha)) {
    $script:Passed++
    Write-Output "PASS T02-promoted-digest sha256=$redSha"
  } else {
    $script:Failed++
    Write-Output 'FAIL T02-promoted-digest'
  }

  function Get-Survivors {
    $leaf = Split-Path -Leaf $bin
    $procs = Get-CimInstance Win32_Process -Filter "Name='$leaf'" -ErrorAction SilentlyContinue
    return @($procs | Where-Object { $_.CommandLine -like '*--stress-sleep-ms 20000*' }).Count
  }

  # T03 - a 64 MiB job cap cannot accommodate a 128 MiB touch
  $d = New-Dir 'T03-memory-probe'
  $r3 = Invoke-Supervisor @('run', '--mem-bytes', 67108864, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T03-memory-probe.json'),
    '--test', 'T03-memory-probe', '--', $bin, '--stress-allocate-bytes', '134217728', '--out', (Join-Path $d 'stage')) (Join-Path $root 'T03-memory-probe.json')
  Check 'T03-memory-probe' $r3 'REFUSED_MEMORY_LIMIT' 'no-promotion'

  # T04 - a 2 s per-process CPU cap stops a 20 s busy loop
  $d = New-Dir 'T04-cpu-probe'
  $r4 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', 2, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T04-cpu-probe.json'),
    '--test', 'T04-cpu-probe', '--', $bin, '--stress-spin-ms', '20000') (Join-Path $root 'T04-cpu-probe.json')
  Check 'T04-cpu-probe' $r4 'REFUSED_CPU_LIMIT' 'no-promotion'

  # T05 - a 2 s wall deadline terminates a 20 s sleep
  $d = New-Dir 'T05-wall-probe'
  $r5 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', 2000,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T05-wall-probe.json'),
    '--test', 'T05-wall-probe', '--', $bin, '--stress-sleep-ms', '20000') (Join-Path $root 'T05-wall-probe.json')
  Check 'T05-wall-probe' $r5 'REFUSED_WALL_TIMEOUT' 'no-promotion'

  # T06 - a descendant cannot outlive the job
  $d = New-Dir 'T06-descendant'
  $r6 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', 2000,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T06-descendant.json'),
    '--test', 'T06-descendant', '--', $bin, '--stress-spawn-descendant', '20000', '--out', (Join-Path $d 'stage')) (Join-Path $root 'T06-descendant.json')
  Check 'T06-descendant' $r6 'REFUSED_WALL_TIMEOUT' 'no-promotion'
  Start-Sleep -Milliseconds 500
  $survivors = Get-Survivors
  if ($survivors -eq 0) {
    $script:Passed++
    Write-Output 'PASS T06-descendant-containment surviving_descendants=0'
  } else {
    $script:Failed++
    Write-Output "FAIL T06-descendant-containment surviving_descendants=$survivors"
  }

  # T07 - the 20 MiB input ceiling refuses before the job is created
  $d = New-Dir 'T07-input-boundary'
  $big = Join-Path $d 'oversize.bin'
  $fs = [System.IO.File]::Create($big)
  $fs.SetLength(20971521)
  $fs.Close()
  $r7 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T07-input-boundary.json'),
    '--test', 'T07-input-boundary', '--input', $big, '--', $bin, '--case', 'neg:empty', '--out', (Join-Path $d 'stage'), '--json', (Join-Path $d 'stage/case.json')) (Join-Path $root 'T07-input-boundary.json')
  Check 'T07-input-boundary' $r7 'REFUSED_INPUT_TOO_LARGE' 'no-child'

  # T08 - a post-decode digest mismatch promotes nothing
  $d = New-Dir 'T08-digest-mismatch'
  $wrong = 'f' * 64
  $r8 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T08-digest-mismatch.json'),
    '--test', 'T08-digest-mismatch', '--case-file', $redFile, '--expect-sha256', $wrong,
    '--', $bin, '--case', 'pos:tests/fixtures/red.avif', '--pkg', $pkg, '--out', (Join-Path $d 'stage'), '--json', (Join-Path $d 'stage/case.json')) (Join-Path $root 'T08-digest-mismatch.json')
  Check 'T08-digest-mismatch' $r8 'REFUSED_DIGEST_MISMATCH' 'no-promotion'

  # T09 - a workload that cannot be created is a typed refusal
  $d = New-Dir 'T09-child-creation'
  $r9 = Invoke-Supervisor @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T09-child-creation.json'),
    '--test', 'T09-child-creation', '--', (Join-Path $d 'no-such-workload-xyz.exe')) (Join-Path $root 'T09-child-creation.json')
  Check 'T09-child-creation' $r9 'REFUSED_CHILD_CREATION' 'no-promotion'

  # T10 - an unusable containment configuration refuses to run anything
  $d = New-Dir 'T10-container-unavailable'
  $r10 = Invoke-Supervisor @('run', '--mem-bytes', 0, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
    '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', (Join-Path $root 'T10-container-unavailable.json'),
    '--test', 'T10-container-unavailable', '--', $bin, '--report-limits', '--out', (Join-Path $d 'stage')) (Join-Path $root 'T10-container-unavailable.json')
  Check 'T10-container-unavailable' $r10 'REFUSED_CONTAINER_UNAVAILABLE' 'no-child'

  # T11 - the whole accepted corpus, inside the normal job limits
  $ids = @(& $bin --list --pkg $pkg)
  $nOk = 0
  $nBad = 0
  foreach ($id in $ids) {
    if (-not $id) { continue }
    $safe = (($id -replace '[^A-Za-z0-9]', '_') -replace '_+$', '')
    $d = New-Dir "T11-corpus/$safe"
    $recPath = Join-Path $root "T11-$safe.json"
    $supArgs = @('run', '--mem-bytes', $normalMem, '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
      '--stage', (Join-Path $d 'stage'), '--final', (Join-Path $d 'final'), '--json', $recPath, '--test', "T11-corpus:$id")
    if ($KnownDigests.ContainsKey($id)) {
      $supArgs += @('--case-file', "$safe.planes.bin", '--expect-sha256', $KnownDigests[$id])
    }
    $supArgs += @('--', $bin, '--case', $id, '--pkg', $pkg, '--out', (Join-Path $d 'stage'), '--json', (Join-Path $d 'stage/case.json'))
    $rec = Invoke-Supervisor $supArgs $recPath
    $caseStatus = ''
    $caseJson = Join-Path $d 'final/case.json'
    if (Test-Path -LiteralPath $caseJson) { $caseStatus = (Get-Content -LiteralPath $caseJson -Raw | ConvertFrom-Json).status }
    if ($null -ne $rec -and $rec.outcome -eq 'OK' -and $caseStatus -eq 'PASS') {
      $nOk++
    } else {
      $nBad++
      $supOutcome = if ($null -ne $rec) { $rec.outcome } else { 'none' }
      Write-Output "FAIL T11-corpus case=$id supervisor=$supOutcome case_status=$caseStatus"
    }
  }
  if ($nBad -eq 0 -and $nOk -eq 25) {
    $script:Passed++
    Write-Output "PASS T11-corpus cases=$nOk digests=12/12 refusals=13/13"
  } else {
    $script:Failed++
    Write-Output "FAIL T11-corpus ok=$nOk bad=$nBad expected 25"
  }

  # T12 - two simultaneous contained workloads each stay inside their own policy
  $d = New-Dir 'T12-parallel'
  $mk = {
    param($sfx)
    @('-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', $PSCommandPath, 'run',
      '--mem-bytes', '67108864', '--cpu-sec', $normalCpu, '--wall-ms', $normalWall,
      '--stage', (Join-Path $d "stage$sfx"), '--final', (Join-Path $d "final$sfx"),
      '--json', (Join-Path $root "T12$sfx.json"), '--test', "T12-parallel-$sfx", '--',
      $bin, '--stress-allocate-bytes', '134217728', '--out', (Join-Path $d "stage$sfx"))
  }
  $pa = Start-Process -FilePath $hostExe -ArgumentList (& $mk 'A') -PassThru -WindowStyle Hidden
  $pb = Start-Process -FilePath $hostExe -ArgumentList (& $mk 'B') -PassThru -WindowStyle Hidden
  $recA = Join-Path $root 'T12A.json'
  $recB = Join-Path $root 'T12B.json'
  for ($n = 0; $n -lt 160; $n++) {
    if ((Test-Path -LiteralPath $recA) -and (Test-Path -LiteralPath $recB)) { break }
    Start-Sleep -Milliseconds 250
  }
  try { $null = $pa.WaitForExit(30000) } catch { }
  try { $null = $pb.WaitForExit(30000) } catch { }
  $r12a = $null
  $r12b = $null
  if (Test-Path -LiteralPath $recA) { $r12a = (Get-Content -LiteralPath $recA -Raw).Trim() | ConvertFrom-Json }
  if (Test-Path -LiteralPath $recB) { $r12b = (Get-Content -LiteralPath $recB -Raw).Trim() | ConvertFrom-Json }
  Check 'T12-parallel-A' $r12a 'REFUSED_MEMORY_LIMIT' 'no-promotion'
  Check 'T12-parallel-B' $r12b 'REFUSED_MEMORY_LIMIT' 'no-promotion'
  $residue = @(Get-ChildItem -LiteralPath (Join-Path $d 'finalA') -File -ErrorAction SilentlyContinue).Count +
             @(Get-ChildItem -LiteralPath (Join-Path $d 'finalB') -File -ErrorAction SilentlyContinue).Count
  if ($residue -eq 0) {
    $script:Passed++
    Write-Output 'PASS T12-parallel-residue both parallel workloads refused with zero promotion'
  } else {
    $script:Failed++
    Write-Output "FAIL T12-parallel-residue promoted_files=$residue"
  }

  Write-Output "suite-totals passed=$script:Passed failed=$script:Failed"
  if ($script:Failed -gt 0) { $script:ExitCode = 1 } else { $script:ExitCode = 0 }
}

$script:ExitCode = 2
$mode = ''
if ($Arguments.Count -gt 0) { $mode = $Arguments[0] }
$rest = @()
if ($Arguments.Count -gt 1) { $rest = @($Arguments[1..($Arguments.Count - 1)]) }
$script:Arguments = $rest
switch ($mode) {
  'run' { $null = Invoke-Run; exit 0 }
  'suite' { Invoke-Suite; exit $script:ExitCode }
  default { Write-Error 'usage: limit-windows.ps1 run|suite -- ...'; exit 2 }
}
