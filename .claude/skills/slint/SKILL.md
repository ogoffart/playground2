---
name: slint
description: Expert guidance for building, debugging, and working with Slint GUI applications. Covers the .slint markup language, project setup, layout/sizing rules, common compile-error gotchas, input handling, custom drawing, Rust/C++/JS/Python interop, the language server, and the embedded MCP server for runtime inspection.
---

# Slint Development Skill

Use this skill when building, debugging, or reviewing applications that use [Slint](https://slint.dev), a declarative GUI toolkit for native user interfaces across desktop, embedded, mobile, and web platforms.

## When to Use This Skill

- Writing or debugging `.slint` files
- Integrating Slint with Rust, C++, JavaScript, or Python
- Investigating layout, binding, rendering, or event-handling issues
- Enabling the Slint MCP server for runtime inspection and UI debugging
- Explaining or reviewing Slint-specific code patterns

## How to Help

- Prefer idiomatic Slint patterns over manual UI workarounds.
- Match guidance to the user's language binding and Slint version.
- Reach for the reference files below — most first-time compile errors and
  "why doesn't this fill / why is my padding ignored" questions are answered in
  `reference/gotchas.md` and `reference/language-and-layout.md`.
- Suggest the MCP server when runtime inspection or interaction would help.
- When unsure about an element/property, check the version's reference docs
  (see **Documentation Reference**) rather than guessing — the API is small and
  precise.

## Reference Files (read on demand)

This entry point is intentionally short. Open the relevant file when the task
calls for it:

| File | Read it when… |
|---|---|
| `setup.md` | Starting a project or wiring the build (Rust/C++/Node/Python). |
| `reference/language-and-layout.md` | Writing components; deciding layouts; an element won't size/fill as expected. |
| `reference/gotchas.md` | A `.slint` file won't compile, or colors/units/math/rotation/enums behave oddly. |
| `reference/events-and-overlays.md` | Handling clicks/keys/modifiers, or building popovers/menus/context menus. |
| `reference/drawing-and-theming.md` | Drawing custom vector graphics with `Path`, or doing light/dark theming. |
| `reference/interop.md` | Connecting the UI to host-language logic (models, callbacks, globals). |
| `reference/debugging-and-mcp.md` | Debugging at runtime, headless/CI rendering, or driving the app via the MCP server. |
| `lsp-install.md` | Installing the `slint-lsp` language server. |

## `.slint` in 30 seconds

`.slint` files are declarative and reactive: a property binding is an expression
that re-evaluates automatically when anything it reads changes.

```slint
import { Button, VerticalBox } from "std-widgets.slint";

component Counter inherits Rectangle {     // root element decides fill behavior
    in property <string> label;            // parent/host writes
    out property <int> count;              // component writes
    callback changed(int);                 // notify the outside world

    VerticalBox {
        Text { text: "\{root.label): \{root.count)"; }   // interpolation
        Button { text: "+"; clicked => { root.count += 1; root.changed(root.count); } }
    }
}
```

Property directions: `in` / `out` / `in-out` / `private`. Two-way bind with
`a <=> b`. Conditionals/loops: `if cond : E {}`, `for it[i] in model : E {}`.
Singletons for shared state and host interop: `export global Foo { ... }`. Run
one-time code with `init => { ... }`. The deeper material is in the reference
files above.

## Documentation Reference

Full docs for the latest version: https://slint.dev/docs — the Language guide
(concepts, syntax, patterns), the Reference (elements, properties, types,
standard widgets), Language integrations (Rust, C++, Node.js, Python), and
Tutorials. For a specific version use
`https://releases.slint.dev/<version>/docs` (e.g. `…/1.15.1/docs`). When you need
the exact signature of an element or property, consult these rather than guessing.
