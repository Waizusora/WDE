# Licensing and dependency policy

## Project license

WDE's original source code is licensed under the Mozilla Public License 2.0 (MPL-2.0). The full license text is in the repository root `LICENSE`.

## Binding project dependency policy

WDE follows the [Wicofe Dependency License Policy 0.1](https://github.com/Waizusora/Wicofe-ThirdParty/blob/main/policies/DEPENDENCY_LICENSE_POLICY_0.1.md). The rules are:

- GPL dependencies are prohibited.
- AGPL dependencies are prohibited.
- LGPL is conditionally eligible only when the dependency is essential, cannot reasonably be implemented in-house, the exact LGPL version and integration are reviewed, all applicable obligations are met, and the project owner explicitly approves the scoped use.
- Third-party dependencies under all other license families are disallowed unless the canonical policy is formally amended.
- Unknown, ambiguous or incomplete license evidence blocks approval.

Do not use a permissive-license allowlist as an automatic approval mechanism. A dependency is an exception, not a default. Evaluate the full transitive dependency closure, enabled features, generated code and shipped assets.

## Required review for every proposed dependency

Before proposing a dependency:

1. State the concrete WDE requirement and why it is essential.
2. Explain why the capability cannot reasonably be implemented in-house.
3. Evaluate alternatives and explain why no suitable Wicofe-owned or non-LGPL option meets the requirement.
4. Inspect the exact source version, license texts and copyright notices.
5. Inspect all transitive dependencies and optional/target-specific features.
6. Review bundled assets, fonts, protocol definitions and generated files.
7. Record source provenance, hashes, patches, security status and distribution obligations.
8. Obtain Wicofe-ThirdParty review and explicit project-owner approval for an LGPL exception.

A successful build, a top-level crate license or a protocol's license does not prove that the entire dependency closure is approved.

## Wayland note

The Wayland protocol and individual implementation libraries are separate things. Review the license of each library and each protocol extension actually shipped. Do not infer the license of a dependency from the protocol name.
