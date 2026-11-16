# Licensing and dependency policy

## Project license

WDE's original source code is licensed under MPL-2.0. The full license text is in the repository root `LICENSE`.

## Binding project dependency policy

WDE follows the [Wicofe Dependency License Policy 0.2](https://github.com/Waizusora/Wicofe-ThirdParty/blob/main/policies/DEPENDENCY_LICENSE_POLICY_0.2.md):

- GPL dependencies are prohibited.
- AGPL dependencies are prohibited.
- LGPL is conditionally allowed after exact-version technical/license review and scoped approval. Prefer in-house implementation when reasonably feasible.
- Other licenses are allowed when their exact terms, compatibility and obligations are reviewed and met. Examples include MIT, BSD, ISC, Apache-2.0, MPL-2.0 and Zlib.
- Other copyleft, custom, proprietary or ambiguous terms require careful compatibility review and explicit project-owner approval.
- Unknown or incomplete license evidence blocks approval pending review.

Do not treat any license label as automatic approval. Evaluate the complete transitive dependency closure, enabled features, generated code and shipped assets. Preserve required notices, attributions, patent terms and source/modification obligations.

## Required review for every proposed dependency

1. State the concrete WDE requirement and why the dependency is useful or necessary.
2. Assess whether an in-house implementation is reasonably feasible and compare maintenance/security costs.
3. Evaluate alternatives and explain the selected option.
4. Inspect exact source version, license texts and copyright notices.
5. Inspect all transitive dependencies and optional/target-specific features.
6. Review bundled assets, fonts, protocol definitions and generated files.
7. Record source provenance, hashes, patches, security status and distribution obligations.
8. Obtain Wicofe-ThirdParty review and project-owner approval for LGPL, other copyleft, custom or proprietary terms.

A successful build, a top-level crate license or a protocol's license does not prove that the entire dependency closure is approved.

## Cross-repository audit snapshot — 2026-11-16

The latest completed 29-repository evidence run recorded WDE at commit `471c66ed8951295d9f8533c0a43c786ffe552074`:

| WDE finding | Result |
|---|---:|
| Dependency/build-time records | 239 |
| NEEDS_REVIEW | 220 |
| UNKNOWN | 19 |
| REJECTED | 0 |

No GPL/AGPL record was identified in the WDE records in this snapshot. This is **not** a PASS or compliance approval: 220 records still require scoped human review, and 19 records remain UNKNOWN. The records include the resolved Cargo dependency closure and build/CI dependencies. The exact package versions, evidence, enabled features and parent edges are in the [cross-repository audit artifact](https://github.com/Waizusora/Wicofe-Tools/actions/runs/43878335380/artifacts/14231212277).

The WDE CI workflow pins the Rust toolchain to 1.100.0, pins action source revisions, and captures installed system-package deltas for license inventory. Runner-image and system-package evidence is still incomplete; a pinned runner label or a package delta is not a complete immutable build-environment SBOM.

Re-audit is required when the workspace dependency graph, optional features, target matrix, CI workflow, toolchain, system packages or distribution scope changes. WDE is not approved for release distribution until the applicable records are reviewed and the final image inventory is reconciled.

## Wayland note

The Wayland protocol and individual implementation libraries are separate things. Review the license of each library and each protocol extension actually shipped. Do not infer the license of a dependency from the protocol name.
