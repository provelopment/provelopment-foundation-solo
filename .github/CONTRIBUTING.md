# Contributing to Foundation Solo

Foundation Solo is an independent open-source foundation for exactly one Entity. Contributions should keep its EFS model understandable and first-class, not reintroduce multi-Entity publishing or hidden Spoke layers.

## Before contributing

1. Read the root [README](../README.md), [AGENTS](../AGENTS.md), and the authoritative [product](../.documents/About/FOUNDATION-SOLO.md) and [architecture](../.documents/Architecture/ARCHITECTURE.md) documents.
2. For a substantive behavioral or architectural change, open a discussion/issue **if that channel exists**, or request maintainer review before investing in a large implementation. A proposal or issue is not authorization for the project's confined implementation agent.
3. Keep pull requests small and coherent. Clearly explain the user-observable change, EFS effect, routing effect, and why additional complexity is necessary.
4. Add focused tests for positive and refusal cases. Never claim a test passed unless it was run. Once the repository has an accepted toolchain, follow the **actual** documented commands; none are assumed in this initial scaffold.
5. Do not commit secrets, private location/contact data, generated outputs, dependencies or artwork you cannot license. Describe third-party material and provenance in the pull request.

## Review and approval

The Owner/maintainers decide acceptance, merges and releases. An external contribution is a proposal until reviewed; it is not a work order for the dedicated implementation agent. That agent is governed by the Solo [implementation SOP](../.documents/SOP/sop-implementation.md) and the sole active WIP work order.

## License and provenance

The Owner has approved **Apache License 2.0** for Foundation Solo and the full license is supplied in the root `LICENSE`. Any public release or contribution workflow remains separately subject to the repository's verification and authorization requirements. Contributors retain rights in their original work and must be able to submit it under the eventual project license. A contribution must not import code/assets from another repository without license and attribution review.

## Conduct and security

See [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md) and [SECURITY](SECURITY.md). Security vulnerabilities must not be disclosed in a public issue while the maintainers establish a confidential report mechanism.
