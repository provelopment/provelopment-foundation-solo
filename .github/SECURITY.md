# Security Policy — Foundation Solo

> **Status.** Foundation Solo is a **public source repository** with **no published application versions yet**. GitHub **private vulnerability reporting is enabled** for this repository and has been independently verified. No released, supported or deployed version exists to patch.

## Responsible reporting

- **Preferred channel:** this repository's **private vulnerability reporting** — GitHub **Security** → **Advisories** → **Report a vulnerability** (direct link: <https://github.com/provelopment/provelopment-foundation-solo/security/advisories/new>). Reports remain private between the reporter and the maintainer.
- Do **not** publish exploit details, exposed credentials or nonpublic data in a public issue, pull request or discussion. No public issue is an appropriate place for vulnerability details.
- Include the affected revision or commit, reproduction steps, impact, and the minimum necessary evidence. Do not access third-party systems without authorization.

## Supported versions

**No application version is supported yet**, because none has been released. The Owner-approved **future** support policy is recorded canonically in [ARCHITECTURE.md §6C.3](../.documents/Architecture/ARCHITECTURE.md): official support covers the latest stable product minor release series and its **immediately preceding stable series for 90 calendar days** after replacement by a successor; older series carry no indefinite backport promise, and no LTS is promised today.

**These are non-guaranteed, best-effort targets that do not apply until a first release exists.** Once support begins, the intended response targets are acknowledgement of a private report within **5 business days**, initial validity/impact/severity triage within **10 business days**, urgent assessment of credible active exploitation or a critical issue begun within **2 business days**, and subsequent reporter updates at least every **14 calendar days** while the matter is unresolved. Remediation is risk-based with **no fixed fix deadline**, and there is no 24/7 or contractual SLA. Disclosure is coordinated with the reporter once a verified fix or mitigation and useful adopter guidance exist.

**Factual prerequisites not yet met.** A named and accountable **Security Maintainer** must be designated before the first release, and the business-day calendar, timezone and published supported-version status must be recorded and linked to immutable release identities. None exists today, and nothing in this file implies that support has begun.

## Scope

Solo's public content is not authenticated by default. An **unlisted** Page remains accessible to anyone with its exact URL and must never hold secrets. Location is an expressly authored EFS property, not a source of routing or authorization. Report any privacy leakage or unlisted-content discovery regression as a potential security issue.
