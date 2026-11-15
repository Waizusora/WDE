# WDE Architecture

## Scope

WDE is a modular desktop environment. The complete Wicofe installation may bundle all standard components, while standalone builds can select only the components they need.

## Runtime model

- The compositor owns the Wayland display, input dispatch, output configuration, surface composition, focus, and window state.
- The panel and launcher are separate client processes.
- The session component starts and supervises the desktop applications for a user session. System service supervision remains the responsibility of Osynit.
- Settings are stored outside the compositor and applied through explicit interfaces.
- Shared crates contain stable types and small utilities, not hidden global state.

## Compositor and window manager

The first implementation combines compositor and window-management responsibilities in `wde-compositor`. A separate window-manager process is not required for the initial release.

The initial window lifecycle is:

1. A client connects to the Wayland display.
2. The compositor receives and configures its surface.
3. The compositor tracks focus and window state.
4. The client can be minimized, maximized/restored, or closed through the window controls.
5. Minimized windows remain represented in the panel task list.

The three circular controls are ordered close (red), maximize/restore (yellow), minimize (green). Their accessible names and keyboard alternatives must exist even though the visual controls have no glyphs.

## Display protocol and framework

Wayland is the display protocol target. Smithay is a candidate compositor framework, subject to a feature-by-feature and full dependency-tree license audit before it is added to the workspace.

Keep WDE-facing interfaces independent of Smithay-specific types where practical. This preserves the option to replace implementation details later.

## Visual system

- Solid black and solid white backgrounds.
- Monochrome surfaces and text.
- One persistent bottom panel by default.
- No top dock, desktop shortcuts, desktop widgets, blur, or transparency by default.
- Motion and effects are opt-in and must not be required for core operation.

## Portability

WDE should avoid a hard dependency on Osynit. Session startup and shutdown integration must use documented interfaces so that WDE can be packaged for compatible standalone environments.

Portability is constrained by the selected compositor backend, graphics stack, input stack, and system integration. It must be verified per target rather than assumed.

## Performance

Measure rather than guess:

- Idle resident memory.
- Idle CPU utilization.
- Startup-to-ready time.
- Window creation and state-transition latency.
- Frame pacing under representative workloads.
- Number of resident processes.

Prefer event-driven communication over polling and avoid unnecessary background services.
