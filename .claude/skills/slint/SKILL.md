---
name: slint
description: Expert guidance for building, debugging, and working with Slint GUI applications. Covers the .slint markup language, project setup, layout/sizing rules, common compile-error gotchas, input handling, custom drawing, Rust/C++/JS/Python interop, the language server, and the embedded MCP server for runtime inspection.
---

# Slint Development Skill

Use this skill when building, debugging, or reviewing applications that use [Slint](https://slint.dev), a declarative GUI toolkit for native user interfaces across desktop, embedded, mobile, and web platforms.

## When to Use This Skill

Use this skill when the task involves:
- Writing or debugging `.slint` files
- Integrating Slint with Rust, C++, JavaScript, or Python
- Investigating layout, binding, rendering, or event-handling issues
- Enabling the Slint MCP server for runtime inspection and UI debugging
- Explaining or reviewing Slint-specific code patterns

## How to Help

- Prefer idiomatic Slint patterns over manual UI workarounds.
- Match guidance to the user's language binding and Slint version.
- Watch for the common pitfalls listed under **Gotchas** below — most first-time
  compile errors and "why doesn't this fill / why is padding ignored" questions
  are covered there.
- Suggest the MCP server when runtime inspection or interaction would make
  debugging easier.
- Prefer solutions that preserve Slint's declarative and reactive model.
- When unsure about an element/property, check the version's reference docs
  (see **Documentation Reference**) rather than guessing — the API surface is
  precise and small.

## The `.slint` Language — Essentials

`.slint` files are declarative and reactive: a property binding is an expression
that is automatically re-evaluated when anything it reads changes.

```slint
import { Button, VerticalBox } from "std-widgets.slint";

// A reusable component. The root element decides the component's nature
// (a Rectangle fills its parent; a layout sizes to its content — see Sizing).
component Counter inherits Rectangle {
    in property <string> label;          // set by the parent / Rust
    out property <int> count;            // readable by the parent
    in-out property <bool> enabled: true;// read+write both sides
    private property <int> step: 1;      // internal only
    callback changed(int);               // notify the outside world

    background: area.has-hover ? #eee : transparent;  // reactive binding

    VerticalBox {
        Text { text: "\{root.label): \{root.count)"; }  // string interpolation
        Button {
            text: "+";
            clicked => { root.count += root.step; root.changed(root.count); }
        }
    }
    area := TouchArea { }                 // `name :=` gives an element an id
}
```

Key constructs:
- **Property directions**: `in` (parent/Rust writes), `out` (component writes),
  `in-out` (both), `private` (internal). Plain `property` defaults to private-ish;
  be explicit for anything crossing a boundary.
- **Two-way binding**: `width <=> other.width;` keeps two properties in sync.
- **Callbacks**: `callback foo(int, string);` then `foo => { ... }` or
  `self.foo(1, "x")`. Callbacks may return values and have a body.
- **Conditionals & loops**: `if cond : Elem { }` and
  `for item[index] in model : Elem { }`. `for i in 5 : ...` iterates `0..4`.
- **`@children`**: forward injected children into a placeholder inside a component.
- **Globals**: `export global Foo { ... }` — singletons for shared state, theme
  tokens, and the Rust/JS/Python interop surface (see **Connecting to business
  logic**).
- **`init => { ... }`** runs once when an element is created (handy to call
  `some-focus-scope.focus()`).

## Layout & Sizing (read this before fighting the layout)

- Put elements in layouts (`VerticalLayout`, `HorizontalLayout`, `GridLayout`, or
  the padded `*Box` widgets) instead of positioning by hand. Use `x`/`y` only for
  overlays, popovers, and custom-drawn content.
- **`padding` and `spacing` only do something on layout elements.** Setting
  `padding-left` on a `Text` or `Rectangle` is silently ignored (the compiler
  warns). To inset a `Text`, wrap it in a `HorizontalLayout { padding-left: 6px; Text {...} }`.
- **Fill vs. preferred size.** Built-ins like `Rectangle`, `TouchArea`,
  `FocusScope`, and `Flickable` expand to fill their parent by default; `Text`,
  `Image`, `Path`, and layouts take their *implicit/preferred* size. **A custom
  component or a layout placed inside a non-layout parent (a `Rectangle`, or the
  `Window`) will NOT stretch to fill — it ends up at preferred size, often
  centered.** Fix it by giving the child `width: 100%; height: 100%`, or by making
  the parent a layout, or by making the layout the component's own root.
- Inside a layout use `horizontal-stretch` / `vertical-stretch` and
  `min/preferred/max-width/height` to control distribution; a `Rectangle { }` with
  stretch makes a flexible spacer.
- `Window` (and `Dialog`) is the only valid top-level exported element for an
  application entry point.

## Gotchas & Common Compile Errors

These bite almost everyone at least once:

- **Units.** `length` (`px`, `pt`, `rem`, …) and `int`/`float` are distinct types.
  Convert with `value * 1px` or `len / 1px`. `length`-typed properties like
  `letter-spacing` reject unitless/`em` values — use `px` (e.g. `0.4px`, not
  `0.04em`).
- **Colors.** No OKLCH/HSL literals. Use `#rgb`, `#rrggbb`, `#rrggbbaa`,
  `rgb(r,g,b)`, or `rgba(r,g,b,a)` where `a` is `0.0..1.0`. Convert design tokens
  to hex/rgba ahead of time. `.brighter(f)` / `.darker(f)` / `.with-alpha(a)`
  adjust a color.
- **Math functions** come in two callable forms (not bare names, generally):
  - methods on a number: `x.floor()`, `x.ceil()`, `x.round()`, `x.sqrt()`,
    `x.mod(y)`, `x.abs()`, `x.clamp(lo, hi)`, `x.max(y)`, `x.min(y)`,
    `x.to-fixed(2)` (→ string), `x.to-precision(3)`;
  - the `Math` namespace: `Math.max(a, b)`, `Math.min`, `Math.clamp`,
    `Math.round`, `Math.floor`, `Math.pow`, `Math.sin`, `Math.atan2`, …
  Use these instead of guessing a bare `floor(...)`. Integer division yields a
  float, so wrap with `.floor()` when assigning to an `int`.
- **Rotation** (`rotation-angle`) exists only on `Image` and `Text`, **not** on
  `Rectangle` or arbitrary components. To "rotate" something else, rotate an
  `Image`, swap to a pre-rotated glyph, or express the effect another way
  (e.g. a gradient for diagonal stripes, a flipped path for a chevron).
- **Enum values** are written `EnumName.value`, usually lowercase for builtins:
  `PointerEventKind.down`, `PointerEventButton.right`, `ColorScheme.dark`. Special
  keys use the `Key` namespace with CamelCase: `Key.Return`, `Key.Escape`,
  `Key.UpArrow`, `Key.Backspace`.
- **Gradients**: `@linear-gradient(angle, color stop%, …)` and
  `@radial-gradient(circle, color stop%, …)`. Radial is centered-circle only — it
  can't be offset/sized like CSS. Repeating patterns can be faked with hard color
  stops.
- **Animations**: `animate width { duration: 200ms; easing: cubic-bezier(0.4,0,0.2,1); }`.
  You animate *properties*, declared inside the element whose property changes.

## Input Handling

- **`TouchArea`**: `clicked => {}` is modifier-agnostic. For modifier- or
  button-aware handling use `pointer-event(ev)`:
  ```slint
  TouchArea {
      pointer-event(ev) => {
          if (ev.kind == PointerEventKind.down) {
              if (ev.button == PointerEventButton.right) { /* context menu */ }
              if (ev.modifiers.control || ev.modifiers.meta) { /* multi-select */ }
              if (ev.modifiers.shift) { /* range select */ }
          }
      }
      double-clicked => { /* open */ }
  }
  ```
  Useful members: `has-hover`, `pressed`, `mouse-x`/`mouse-y` (local),
  `absolute-position` (window coords — see overlays), `mouse-cursor`.
- **`FocusScope`** for keyboard input. It must hold focus to receive keys — call
  `myscope.focus()` (often in `init`). Handler returns `accept`/`reject`:
  ```slint
  FocusScope {
      key-pressed(e) => {
          if (e.text == Key.Escape) { /* … */ return accept; }
          if ((e.modifiers.control || e.modifiers.meta) && e.text == "a") { return accept; }
          return reject;
      }
  }
  ```
- Clicking a `TextInput`/widget moves focus away from your `FocusScope`; refocus
  it when appropriate (e.g. on a background click) so shortcuts keep working.

## Overlays, Popovers & Context Menus

- `PopupWindow` (and the `ContextMenu`/`MenuBar` widgets) handle the common cases
  with auto-dismiss. For full control over position (e.g. a menu at the exact
  cursor location, or anchored to a toolbar button), a robust pattern is a manual
  overlay:
  - Render the menu as a child of the top-level `Window` (so its `x`/`y` are in
    window coordinates), gated by an `if open : …`.
  - Add a full-window backdrop `TouchArea` behind it that closes the menu on click.
  - Anchor to a widget with `widget.absolute-position.x/.y` (+ its height); clamp
    with `Math.min(x, root.width - menu.width - 8px)` to keep it on screen.
- A custom popover panel (a `Rectangle` placed directly under `Window`) will
  default to filling the window — set its `height` to its content
  (`height: layout.preferred-height;`) so it sizes correctly.

## Custom Drawing with `Path`

`Path` renders vector graphics from an SVG-style command string — great for icons.

```slint
Path {
    width: 24px; height: 24px;
    viewbox-width: 24; viewbox-height: 24;   // command coordinate space
    commands: "M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z";
    stroke: black; stroke-width: 1.2px; fill: transparent;
}
```

- `commands` accepts the SVG path mini-language including arcs (`a`/`A`); multiple
  sub-paths (multiple `M`) are fine. Lines/rects/circles must be expressed as path
  commands (a circle ≈ two `a` arcs).
- The path is scaled to fit `width`×`height` using the viewbox, but **`stroke-width`
  is a logical length and is *not* scaled by the viewbox.** To mimic an SVG whose
  stroke is `S` in a `V`-unit viewbox rendered at size `D`, set
  `stroke-width: S * D / V` (e.g. `1.6 * size / 24px`).
- A single `Path` has one `fill` and one `stroke`. For a glyph that mixes filled
  and stroked sub-shapes, use two overlapping `Path`s (one stroked, one filled).
- Codegen tip: when you have many SVG icons, generate the `commands` strings
  (and a `name → commands` lookup function in a global) from your source rather
  than hand-translating.

## Theming & Light/Dark

- `Palette.color-scheme` (from `std-widgets.slint`) reflects the OS light/dark
  setting and updates live; it's also settable to force a scheme for native
  widgets.
- A clean pattern: one `export global Theme` holding every color/length token as
  `out property`s selected by a `dark` bool, e.g.
  `out property <brush> bg: dark ? #1e2025 : #ffffff;` Bind `dark` to the palette
  with an optional user override:
  `out property <bool> dark: pref == 2 ? true : pref == 1 ? false : Palette.color-scheme == ColorScheme.dark;`
  Every component then reads `Theme.bg` etc., so theme switching is automatic.

## Connecting to Business Logic (Rust / C++ / JS / Python)

The cleanest way to wire a non-trivial app is **two globals**: one carrying data
*into* the UI, one carrying callbacks *out*. This avoids threading dozens of
properties/callbacks through every component.

```slint
// globals.slint
export struct Row { id: int, name: string, tags: [int], selected: bool }
export global AppData {           // Rust pushes models/state in
    in property <[Row]> rows;
    in property <string> status;
}
export global Logic {            // UI calls these; Rust handles them
    callback row-clicked(int, bool, bool);   // index, ctrl, shift
    callback refresh();
}
```

Any component can read `AppData.rows` / call `Logic.row-clicked(...)` directly.
From Rust:

```rust
slint::include_modules!();
use slint::{ModelRc, VecModel};

let ui = AppWindow::new()?;
ui.global::<AppData>().set_status("Ready".into());
ui.global::<AppData>().set_rows(ModelRc::new(VecModel::from(rows))); // Vec<Row>
let weak = ui.as_weak();
ui.global::<Logic>().on_row_clicked(move |i, ctrl, shift| { /* … */ });
ui.run()
```

Naming & type mapping (Rust):
- Kebab-case identifiers become snake_case: `is-folder` → `is_folder`,
  `row-clicked` → callback `on_row_clicked` / invoke setter `set_...`.
- `[T]` array properties become `ModelRc<T>`; build with
  `ModelRc::new(VecModel::from(vec))`. For live updates, keep a `Rc<VecModel<T>>`
  and mutate it (`push`/`set_row_data`) instead of replacing the whole model.
- `string` ↔ `SharedString` (`.into()` from `String`/`&str`); `length`/`float` ↔
  `f32`; `int` ↔ `i32`; `brush`/`color` ↔ `slint::Brush`/`Color`.
- A common split: keep the source of truth and all logic in the host language,
  expose the current view as an already-sorted/filtered model, and let `.slint`
  render + forward interactions. Replace prototype timers with real signals
  (`slint::Timer` for periodic UI work; invoke from other threads via
  `slint::invoke_from_event_loop` / a `Weak` handle).

C++/JS/Python expose the same globals/structs through their idiomatic APIs
(getters/setters and callback registration); the property/callback names follow
each language's conventions.

## Project Setup

### Rust

```toml
# Cargo.toml
[dependencies]
slint = "1.x"

[build-dependencies]
slint-build = "1.x"
```

```rust
// build.rs
fn main() { slint_build::compile("ui/main.slint").unwrap(); }
```

```rust
// main.rs
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = MainWindow::new()?;
    // set up globals, models, callbacks…
    app.run()
}
```

To track unreleased features, depend on git:
`slint = { git = "https://github.com/slint-ui/slint", branch = "master" }` (do the
same for `slint-build`).

### C++

```cmake
find_package(Slint)            # or FetchContent
slint_target_sources(my_app ui/main.slint)
target_link_libraries(my_app PRIVATE Slint::Slint)
```

### Node.js

```js
import * as slint from "slint-ui";
const ui = slint.loadFile("ui/main.slint");
const app = new ui.MainWindow();
app.run();
```

### Python

```python
import slint
ui = slint.load_file("ui/main.slint")
app = ui.MainWindow()
app.run()
```

Note: the `slint` wheel's `requires-python` tracks recent CPython releases and
advances with new Slint versions. If `uv add` / `pip install` picks an older Slint
than expected, check the latest wheel's `requires-python` on PyPI and bump your
project's `requires-python` (and `.python-version` for uv) to match before pinning
a Slint version.

## Language Server

`slint-lsp` is the Slint Language Server: diagnostics, hover, go-to-definition,
completion, and formatting for `.slint` files over LSP, usable by any LSP-capable
editor or AI assistant. It is not bundled with this skill — see `lsp-install.md`
in this directory for `cargo install slint-lsp`, prebuilt downloads per platform,
and Linux runtime dependencies.

## Debugging Slint Applications

### Common Issues

1. **Binding loops**: a property depends on itself through a chain. The compiler
   warns; break the cycle with an intermediate property or by restructuring.
2. **Elements not visible**: check `width`/`height` (may be `0` outside a layout —
   see **Layout & Sizing**), `visible`, `opacity`, `clip`, and z-order (later
   siblings render on top).
3. **Layout sizing**: elements outside layouts need explicit `width`/`height`;
   custom components/layouts may need `width/height: 100%` to fill (see Sizing).
4. **Type mismatches**: `length` vs number — convert with `* 1px` / `/ 1px`.
5. **Ignored `padding`/`spacing`**: only effective on layout elements.
6. **Performance**: use `ListView` (not a `for` inside a `ScrollView`) for long
   lists — it virtualizes. Avoid deeply nested `opacity`/`clip` layers.

### Debug Helpers

- `debug("msg", expr)` prints to stderr at runtime.
- `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console` (or `refresh_full_speed`) prints
  frame/perf diagnostics.
- Switch renderer/backend for testing: `SLINT_BACKEND=winit-skia`,
  `winit-femtovg`, or `winit-software`. **`winit-software` is the reliable choice
  for headless/CI/GPU-less machines** (CPU rendering). On headless Linux, run
  under `xvfb-run -a -s "-screen 0 1360x900x24"`; the winit X11 path also needs
  `libxkbcommon-x11` installed.
- `Window::take_snapshot()` (Rust) renders the current window to a pixel buffer —
  handy for a quick screenshot — but for interactive inspection prefer the MCP
  server below.

## MCP Server for AI-Assisted Debugging

Slint (>= 1.17.0) includes an embedded MCP (Model Context Protocol) server that
lets an AI assistant inspect and *drive* a running app in real time: walk the UI
tree, read accessibility properties, take screenshots, and simulate clicks,
drags, typing, and key events. This is the best way to verify real interactions
(selection, navigation, menus) — not just static rendering.

### Enabling the MCP Server

**Step 1**: Build with `SLINT_EMIT_DEBUG_INFO=1` so element IDs and source
locations survive compilation (without it, introspection is far less useful). Set
`SLINT_MCP_PORT` and pass `--features slint/mcp` on the command line:

```sh
SLINT_EMIT_DEBUG_INFO=1 SLINT_MCP_PORT=9315 cargo run --features slint/mcp
```

Do **not** add `mcp` to the `[features]` section of `Cargo.toml` — use the
`--features` flag. On a headless machine, prefix with
`SLINT_BACKEND=winit-software xvfb-run -a -s "-screen 0 1360x900x24"`.

**Step 2**: Connect to `http://localhost:9315/mcp` using Streamable HTTP
(JSON-RPC). When scripting from the shell, `curl` is the most reliable client —
include the `Accept` header for the streamable transport:

```sh
# List tools (confirms the server is up)
curl -s -X POST http://127.0.0.1:9315/mcp \
  -H "Content-Type: application/json" \
  -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}'

# List windows -> returns windowHandle {index, generation}
curl -s -X POST http://127.0.0.1:9315/mcp \
  -H "Content-Type: application/json" -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"list_windows","arguments":{}}}'

# Screenshot (PNG base64 in an image content block). Pipe to a file before
# parsing — the payload is large and breaks naive inline JSON parsing:
curl -s -X POST http://127.0.0.1:9315/mcp \
  -H "Content-Type: application/json" -H "Accept: application/json, text/event-stream" \
  -d '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"take_screenshot","arguments":{"windowHandle":{"index":"1","generation":"1"}}}}' > shot.json
```

### Available Tools (typical)

`list_windows`, `get_window_properties`, `get_element_tree`,
`get_element_properties`, `find_elements_by_id` (qualified id like
`MyComponent::my-button`), `query_element_descendants`, `take_screenshot`,
`click_element`, `drag_element`, `invoke_accessibility_action`,
`set_element_value`, `dispatch_key_event`, and `start`/`stop_event_recording`.
Most tools take element/window handles returned by the tree/`list_windows` calls.

### Tips

- To target elements reliably, give them ids (`foo := Rectangle {}`) and build
  with `SLINT_EMIT_DEBUG_INFO=1`; then `find_elements_by_id` with
  `ComponentName::id`.
- `dispatch_key_event` takes a window handle and `text` (special keys via their
  Slint key codes); `click_element`/`drag_element` take element handles. Drive a
  flow, then `take_screenshot` to verify the resulting state.
- A `.mcp.json` registering an HTTP server at `http://localhost:9315/mcp` lets
  Claude Code attach automatically while the app runs.

### Version Requirements

| Slint Version | MCP Support |
|---------------|-------------|
| < 1.17.0 | Not available |
| >= 1.17.0 | Enable via `--features slint/mcp` on the cargo command line |

### When to Suggest MCP

When the user is debugging layout/visual issues, trying to understand the runtime
element hierarchy, testing interactions programmatically, verifying accessibility
properties, or diagnosing event-handling problems.

## Documentation Reference

Full documentation for the latest version is at https://slint.dev/docs. Key
sections: the Language guide (concepts, syntax, patterns), the Reference
(elements, properties, types, standard widgets), Language integrations (Rust,
C++, Node.js, Python API docs), and Tutorials.

For a specific version: `https://releases.slint.dev/<version>/docs`
(e.g. `https://releases.slint.dev/1.15.1/docs`). When you need the exact
signature of an element or property, consult these rather than guessing.
