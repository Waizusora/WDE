# WDE compositor integration

## Current state

WDE currently contains backend-independent window lifecycle policy and an
optional Smithay 0.7.0 dependency behind the `nested-winit` feature. It does
not yet provide a runnable compositor. The lifecycle model is not itself a
Wayland protocol implementation.

## Integration target

The first executable target is a nested compositor for development and CI
smoke testing. It runs as a normal user-space process inside an existing
Wayland/X11 desktop and opens a host window. It is not the production DRM/TTY
backend.

Smithay 0.7.0's minimal nested example is the API reference for this target.
The required building blocks are:

- `backend::winit` with `GlesRenderer` for the host window and rendering.
- `wayland_server::Display` and `ListeningSocket` for the nested Wayland
  server.
- `CompositorState`, `CompositorHandler`, and
  `delegate_compositor!` for core surface handling.
- `ShmState` / `ShmHandler` and `delegate_shm!` for shared-memory buffers.
- `XdgShellState` / `XdgShellHandler` and
  `delegate_xdg_shell!` for xdg-shell toplevels and popups.
- `SeatState`, `SeatHandler`, and `delegate_seat!` for keyboard/pointer
  seat integration.
- Smithay surface-tree rendering helpers and frame callbacks.

The compositor must own its display, clients, seat, xdg-shell state, renderer
backend, and window policy in one application state object. Client connections
must be dispatched and flushed each event-loop iteration. Surface commits
must invoke Smithay's buffer commit handler.

## WDE behavior requirements

The nested backend is an integration harness, not the final visual design.
The first WDE-managed window implementation must then apply these policies:

- Solid black or white desktop background; no blur or transparency.
- Bottom panel, visible by default; no top dock and no desktop shortcuts.
- Window controls ordered close (red), maximize/restore (yellow), minimize
  (green).
- Minimize hides a window without destroying its Wayland surface.
- Maximize/restore updates xdg-shell configure state and window geometry.
- Closing a window requests client-side close through xdg-shell; it must not
  silently destroy a client-owned surface.
- Focus and task-list state must stay synchronized with mapped toplevels.

## Implementation sequence

1. Add a feature-gated `wde-nested` binary and make the Smithay 0.7.0
   nested backend compile.
2. Implement display/client lifecycle, compositor, shm, xdg-shell, and seat
   handlers.
3. Render committed toplevel surface trees in the nested host window.
4. Add hit-testing, focus, configure handling, and WDE window lifecycle
   integration.
5. Render WDE server-side decorations and the bottom panel.
6. Add an automated headless/protocol smoke test where the CI environment
   supports the required graphics backend.
7. Only after the nested path is stable, design the production DRM/TTY backend.

## Build and license gates

The feature must remain opt-in until its native build requirements are
documented. CI must test both the default workspace and the optional nested
feature on a suitable Linux runner. Before enabling it by default, audit the
full resolved dependency graph and native components for licenses and
redistribution obligations. A crate's top-level license alone is not a
sufficient audit.

## API reference

- Smithay 0.7.0 minimal example:
  https://docs.rs/crate/smithay/0.7.0/source/examples/minimal.rs
- Smithay 0.7.0 xdg-shell handler documentation:
  https://docs.rs/crate/smithay/0.7.0/source/src/wayland/shell/xdg/mod.rs

This document records the integration plan. It does not claim that the nested
compositor has already been implemented or tested.
