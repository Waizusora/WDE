# Wicofe Desktop Environment (WDE)

Wicofe Desktop Environment is an independent, lightweight desktop environment being developed for Wicofe OS. Its design is intentionally minimal: a persistent bottom panel, simple application launching, and restrained window decorations.

**Status:** early development / workspace foundation  
**License:** Mozilla Public License 2.0 (MPL-2.0)

## Design principles

- Minimal interface and low idle resource usage.
- Black and white solid-color themes.
- One bottom panel that remains visible by default.
- No top dock, desktop widgets, or desktop shortcuts by default.
- No blur, transparency, or unnecessary animation.
- Modular components with documented interfaces.
- No copied XFCE source code or other desktop-environment implementation.

## Window controls

The default window-control order is:

1. Red — close.
2. Yellow — maximize / restore.
3. Green — minimize.

Controls are small circles without letter or glyph labels.

## Workspace layout

The Rust workspace is divided into focused crates:

| Crate | Responsibility |
|---|---|
| `wde-core` | Desktop-wide coordination and shared lifecycle |
| `wde-compositor` | Wayland compositor and window management |
| `wde-panel` | Persistent bottom panel |
| `wde-launcher` | Application discovery and launching |
| `wde-session` | Desktop session lifecycle |
| `wde-settings` | User-facing settings |
| `wde-theme` | Black and white themes |
| `wde-protocols` | Internal component interfaces |
| `wde-common` | Shared types and utilities |
| `wde-display` | Display and monitor configuration |

## Build

Requirements: a current stable Rust toolchain with the `rustfmt` and `clippy` components.

```sh
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

The workspace currently contains architectural crate scaffolds. Smithay 0.7.0 is wired as an optional dependency for the `nested-winit` development feature; a working compositor, window decorations, and desktop session are not available yet. Smithay's default features are disabled. The nested backend is a development milestone, not the final direct-to-hardware session backend.

## Dependency and licensing policy

WDE follows the [binding Wicofe dependency license policy](https://github.com/Waizusora/Wicofe-ThirdParty/blob/main/policies/DEPENDENCY_LICENSE_POLICY_0.1.md): GPL and AGPL dependencies are prohibited; LGPL is conditionally eligible only when essential, not reasonably implementable in-house, legally reviewed and explicitly approved by the project owner; all other third-party dependency license families are disallowed unless the policy is formally amended. The same rule applies to transitive dependencies and optional features.

Wayland protocol compatibility does not by itself determine the license of every implementation library or protocol extension. Smithay and its complete enabled/transitive dependency closure must be assessed against the policy before distribution.

## Roadmap

1. Workspace and project policies.
2. Smithay nested backend prototype and dependency license audit.
3. Window lifecycle and circular controls.
4. Persistent bottom panel.
5. Application launcher.
6. Settings, session integration, and performance work.

See [docs/architecture.md](docs/architecture.md) for the initial architecture.

## Documentation index

See [docs/INDEX.md](docs/INDEX.md) for this repository's technical documentation. The [central repository documentation map](https://github.com/Waizusora/Wicofe-Docs/blob/main/REPOSITORY_DOCUMENTATION_MAP.md) identifies canonical ownership and cross-repository boundaries.
