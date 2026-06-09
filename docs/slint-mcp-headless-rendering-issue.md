# MCP server: support headless rendering (screenshots) without a windowing system

## Summary

The embedded MCP server (`--features slint/mcp`, Slint ≥ 1.17) is fantastic for
letting an AI agent inspect and drive a running app. But **taking screenshots
through the MCP server currently requires a real windowing system** (a display /
X11 / Wayland), because the only fully headless backend that hosts the MCP server
— the **testing backend** — has a stub renderer and does not implement
`take_snapshot`.

Request: make `--features slint/mcp` able to **render headlessly**, so
`take_screenshot` works on a bare machine (CI, containers, agents) with no
display, no Xvfb, and no GPU.

## Motivation

The whole point of the MCP server is agent/CI-driven verification of a *running*
app. Three of the four capabilities already work headlessly; screenshots are the
gap:

| Setup | Headless? | Inspect / interact | `take_screenshot` | Real app data |
|---|---|---|---|---|
| MCP on `SLINT_BACKEND=testing` | ✅ no display | ✅ | ❌ *not implemented by the platform* | ✅ |
| MCP on `winit-software` + `xvfb-run` | ⚠️ needs Xvfb + `libxkbcommon-x11` | ✅ | ✅ | ✅ |
| `slint-viewer --screenshot` | ✅ no display | ❌ | ✅ | ❌ (renders the `.slint` standalone; no host logic/globals) |

What's missing is the top-right cell: a **headless software-rendered screenshot of
the real running app**. Today you must fall back to `winit-software` under a
virtual framebuffer, which means installing and wrapping everything in Xvfb
(and `libxkbcommon-x11`) — awkward in CI and for agents.

Notably, the building blocks already exist in-tree: **`slint-viewer` ships exactly
such a headless software backend** (`ScreenshotPlatform` driving a
`SoftwareRenderer`) — see `tools/viewer/screenshot.rs`. It just isn't available
through the `mcp` feature / testing backend.

## Current behavior (reproduction)

1. Build any app with the MCP feature:
   ```sh
   SLINT_EMIT_DEBUG_INFO=1 SLINT_MCP_PORT=9315 cargo run --features slint/mcp
   ```
2. Run it under the windowless testing backend so no display is needed
   (`SLINT_BACKEND=testing`; see the *backend-testing* note below for how it's
   currently reachable).
3. Call the MCP `take_screenshot` tool. It returns:

   ```
   Error: Error grabbing window screenshot:
   WindowAdapter::take_snapshot is not implemented by the platform
   ```

Inspection/interaction tools on the same headless server work fine
(`list_windows`, `get_element_tree`, `click_element`, `dispatch_key_event`, …) —
only rendering is missing.

## Root cause

The MCP server lives in the testing backend, whose window is a stub renderer:

- `internal/backends/testing/testing_backend.rs` — `TestingWindow` implements
  `RendererSealed` itself as a no-op (`impl RendererSealed for TestingWindow`),
  and `renderer(&self) -> &dyn Renderer { self }`. There is **no real rasterizer**,
  so `take_snapshot` is unimplemented.
- The testing backend crate does **not** depend on `i-slint-renderer-software`.
- `internal/backends/testing/mcp_server.rs` dispatches screenshots via
  `dispatch::take_snapshot(...)`, which ultimately calls the window's
  `take_snapshot()` → fails on the stub.

(Line references are from a recent `master`, ~commit `1b7c412`; they may drift.)

## Proposed solution

Make a **software renderer** available to the MCP/testing path so `take_snapshot`
works with no windowing system. Either approach (or both) is fine:

### Option A — give the testing backend a real software renderer (preferred)

Wire `i-slint-renderer-software`'s `SoftwareRenderer` into `TestingWindow` (gated
behind the `mcp` feature, or always) so `TestingWindow::take_snapshot()` renders
to a buffer instead of erroring. The MCP server already lives here, so this
lights up headless screenshots directly, against the real app with real data.

- Add `i-slint-renderer-software` as a (feature-gated) dependency of
  `internal/backends/testing`.
- Have `TestingWindow` own a `SoftwareRenderer`, render the component tree to a
  `SharedPixelBuffer` on demand, and return it from `take_snapshot`.

### Option B — install a headless software backend when `mcp` is enabled and there's no display

Mirror `slint-viewer`'s `ScreenshotPlatform` (`tools/viewer/screenshot.rs`):
a `Platform`/`WindowAdapter` backed by `SoftwareRenderer` (or Skia's software
rasterizer when available). When `--features slint/mcp` is on and no display is
detected (or `SLINT_BACKEND=headless`/similar is set), default to this backend so
`take_screenshot` "just works". `tools/viewer/screenshot.rs::create_renderer()`
is a ready precedent.

### Secondary ask — make the headless backend reachable from the `slint` crate

Today `SLINT_BACKEND=testing` is gated on the selector's `backend-testing`
feature (`internal/backends/selector/lib.rs`: the `"testing" =>` arm), but the
`slint` crate's `mcp` feature only enables `i-slint-backend-selector/mcp`, **not**
`backend-testing`. So selecting the headless backend currently requires an
out-of-band dependency on `i-slint-backend-selector` with its `backend-testing`
feature. Please either:

- have `slint`'s `mcp` feature also enable `backend-testing`, and/or
- re-export a `backend-testing` (or `backend-headless`) feature from the `slint`
  crate,

so the whole thing is a single flag: `cargo run --features slint/mcp`.

## Ideal end state

On a bare CI box / container / agent sandbox, with **no display, no Xvfb, no
GPU**:

```sh
cargo run --features slint/mcp        # auto-selects a headless software backend
# → MCP server up; take_screenshot returns a real PNG of the running app,
#   in addition to the existing inspect/click/type/key tools.
```

One flag → **inspect + interact + screenshot**, fully headless, against the live
application (with its real models/host logic) — strictly more capable than
`slint-viewer --screenshot`, which can only render a `.slint` file standalone.

## Acceptance criteria

- [ ] With `--features slint/mcp` and no windowing system available, the MCP
      `take_screenshot` tool returns a valid PNG of the running window.
- [ ] No Xvfb / X11 / Wayland / GPU required for screenshots.
- [ ] Inspect/interaction tools continue to work headlessly.
- [ ] Selecting the headless backend needs only the `slint` crate (no manual
      dependency on internal `i-slint-backend-*` crates).
- [ ] Documented: how to run the MCP server fully headlessly.

## Alternatives considered

- **`winit-software` + `xvfb-run`** — works today but needs a virtual display and
  `libxkbcommon-x11`; heavyweight for CI/agents and easy to get wrong (e.g. stale
  `/tmp/.X*-lock`).
- **`slint-viewer --screenshot`** — already fully headless, but renders a `.slint`
  file standalone with no host-language logic, so apps whose data lives in
  `global` singletons render their empty/default state. Great for component
  previews, not for screenshotting real application state.

## Pointers (recent `master`)

- `api/rs/slint/Cargo.toml` — `mcp = ["std", "i-slint-backend-selector/mcp"]`
- `internal/backends/selector/Cargo.toml` — `backend-testing`, `mcp` features
- `internal/backends/selector/lib.rs` — `"testing" =>` backend selection;
  `mcp` → `i_slint_backend_testing::mcp_server::init()`
- `internal/backends/testing/testing_backend.rs` — `TestingWindow` stub renderer
- `internal/backends/testing/mcp_server.rs` — `take_screenshot` dispatch
- `tools/viewer/screenshot.rs` — working headless `ScreenshotPlatform` +
  `create_renderer()` (SoftwareRenderer / Skia software) — reuse this pattern
- `internal/renderers/software` — `SoftwareRenderer`

## Environment

- Slint `master` (≥ 1.17), Rust backend, `--features slint/mcp`.
- Linux container, no display server.
