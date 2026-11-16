# WDE Documentation Index

This directory documents Wicofe Desktop Environment architecture, implementation, and user-facing desktop behavior.

## Known design documents
- [Architecture](architecture.md)
- [Licensing policy](licensing.md)

## Scope
WDE owns desktop session behavior, compositor/window management, window decorations, persistent bottom panel, application launcher, settings, themes, and desktop-wide component lifecycle.

Reusable graphics/display infrastructure belongs to [Wicofe-Graphics](https://github.com/Waizusora/Wicofe-Graphics); input policy belongs to [Wicofe-Input](https://github.com/Waizusora/Wicofe-Input); audio services belong to [Wicofe-Audio](https://github.com/Waizusora/Wicofe-Audio). WDE consumes documented userspace interfaces rather than private kernel driver APIs.

## Status
The workspace is an architectural scaffold. Do not describe a functioning compositor, full desktop session, or production-ready shell as available until verified.

- [Central Wicofe documentation index](https://github.com/Waizusora/Wicofe-Docs/blob/main/INDEX.md)
- [Repository documentation map](https://github.com/Waizusora/Wicofe-Docs/blob/main/REPOSITORY_DOCUMENTATION_MAP.md)
