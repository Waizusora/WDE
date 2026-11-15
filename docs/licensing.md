# Licensing and dependency policy

## Project license

WDE's original source code is licensed under the Mozilla Public License 2.0 (MPL-2.0). The full license text is in the repository root `LICENSE`.

## Excluded license families

The current project policy excludes dependencies under:

- GNU General Public License (GPL-2.0, GPL-3.0, and later versions).
- GNU Lesser General Public License (LGPL-2.1, LGPL-3.0, and later versions).
- GNU Affero General Public License (AGPL-3.0 and later versions).

This is a project dependency-selection policy. It does not change the terms of third-party software.

## Candidate permissive licenses

MIT, BSD-2-Clause, BSD-3-Clause, Apache-2.0, ISC, Zlib, and 0BSD may be considered. Compatibility must still be checked against the exact license text, notices, patents, linking model, and distribution method.

MPL-2.0 is the license for WDE's own source files and is not a GNU license.

## Required review for every dependency

Before adding a dependency:

1. Inspect the exact version and its declared license.
2. Inspect its repository license files and copyright notices.
3. Inspect all transitive dependencies.
4. Inspect optional features and target-specific dependencies.
5. Check whether bundled assets, fonts, protocol definitions, or generated files have separate terms.
6. Record the result in the dependency inventory.
7. Re-run the audit whenever dependency versions or enabled features change.

A permissive top-level crate license does not prove that every optional or transitive dependency is acceptable.

## Wayland note

The Wayland protocol and individual implementation libraries are separate things. Review the license of each library and each protocol extension actually shipped. Do not infer the license of a dependency from the protocol name.
