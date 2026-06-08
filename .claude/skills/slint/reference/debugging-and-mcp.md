# Debugging, Headless Rendering & the MCP Server

## Common Issues

1. **Binding loops**: a property depends on itself through a chain. The compiler
   warns; break the cycle with an intermediate property or by restructuring.
2. **Elements not visible**: check `width`/`height` (may be `0` outside a layout —
   see `reference/language-and-layout.md`), `visible`, `opacity`, `clip`, and
   z-order (later siblings render on top).
3. **Layout sizing**: elements outside layouts need explicit `width`/`height`;
   custom components/layouts may need `width/height: 100%` to fill.
4. **Type mismatches**: `length` vs number — convert with `* 1px` / `/ 1px`.
5. **Ignored `padding`/`spacing`**: only effective on layout elements.
6. **Performance**: use `ListView` (not a `for` inside a `ScrollView`) for long
   lists — it virtualizes. Avoid deeply nested `opacity`/`clip` layers.

## Debug Helpers

- `debug("msg", expr)` prints to stderr at runtime.
- `SLINT_DEBUG_PERFORMANCE=refresh_lazy,console` (or `refresh_full_speed`) prints
  frame/perf diagnostics.
- Switch renderer/backend for testing: `SLINT_BACKEND=winit-skia`,
  `winit-femtovg`, or `winit-software`. **`winit-software` is the reliable choice
  for headless/CI/GPU-less machines** (CPU rendering). On headless Linux, run
  under `xvfb-run -a -s "-screen 0 1360x900x24"`; the winit X11 path also needs
  `libxkbcommon-x11` installed.
- `Window::take_snapshot()` (Rust) renders the current window to a pixel buffer —
  handy for a quick screenshot from inside your app — but for interactive
  inspection prefer the MCP server below.

### Screenshotting a `.slint` file headlessly (`slint-viewer --screenshot`)

The simplest way to render a component to an image **with no windowing system and
no app code** is the viewer (Slint >= 1.17):

```sh
slint-viewer --screenshot out.png ui/main.slint          # PNG (or .jpg, or - for stdout)
slint-viewer --screenshot out.png --component MyCard ui/widgets.slint
slint-viewer --screenshot out.png --load-data props.json ui/main.slint
```

- With no `--backend`, the viewer installs its own headless software backend, so
  it works on a bare box (no X/Wayland, no Xvfb). It renders the component at its
  preferred size and exits. `--style`, `-I`/`-L` (include/library paths),
  `SLINT_SCALE_FACTOR`, and `--component` (defaults to the last exported one) all
  apply.
- `--load-data file.json` sets the **root component's** properties before
  rendering — great for previewing data-driven components. It does **not**
  populate `global` singletons, and it doesn't run your host-language logic, so an
  app whose models live in a Rust/C++/JS/Python global will render its default/
  empty state. For a screenshot of real application state, drive the running app
  via the MCP server (`take_screenshot`) or `Window::take_snapshot()` instead.
- Install from a released version with `cargo install slint-viewer` (add
  `--git https://github.com/slint-ui/slint` for unreleased builds). A
  `--no-default-features --features renderer-software` build avoids GPU/windowing
  dependencies.

Rule of thumb: **`slint-viewer --screenshot`** for previewing components/layout/
theme during development; the **MCP server** when you need the running app with
real data and interactions.

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
`--features` flag.

**Headless / no display.** Two options:
- *Screenshots needed*: use the software renderer under a virtual display —
  `SLINT_BACKEND=winit-software xvfb-run -a -s "-screen 0 1360x900x24" cargo run --features slint/mcp`
  (the winit X11 path needs `libxkbcommon-x11`).
- *Inspection/interaction only (no display at all)*: the MCP server is hosted by
  Slint's windowless **testing backend**. Run with `SLINT_BACKEND=testing` and
  the testing backend compiled in. Element-tree queries, `click_element`,
  `dispatch_key_event`, etc. work with no X/Wayland server — but the testing
  backend's renderer is a stub, so **`take_screenshot` returns "not implemented
  by the platform"**. Use this for headless automation/CI of interactions; use the
  software-renderer+Xvfb route when you actually need pixels. (Caveat: the `slint`
  crate doesn't currently re-export the selector's `backend-testing` feature, so
  enabling `SLINT_BACKEND=testing` may require depending on
  `i-slint-backend-selector` with its `backend-testing` feature.)

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
