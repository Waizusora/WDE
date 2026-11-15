# WDE Panel Model

The panel is a separate Wayland client process. Its default placement is the bottom edge and it remains visible. The initial design has no top dock, desktop shortcuts, widgets, blur, or transparency.

The panel model tracks top-level windows as tasks. It stores a stable window identifier, title, normal/maximized/minimized state, and focus indication. The compositor remains authoritative for actual window state; a future Wayland integration layer must synchronize this model from compositor events.

This crate currently implements backend-independent policy and tests. It does not yet create a Wayland surface or render a panel.
