# Input Handling, Overlays & Menus

## Input Handling

- **`TouchArea`**: `clicked => {}` is modifier-agnostic. For modifier/button-aware
  handling use `pointer-event(ev)`:
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
  Other members: `has-hover`, `pressed`, `mouse-x`/`mouse-y` (local),
  `absolute-position` (window coords), `mouse-cursor`.
- **`FocusScope`** for keys — it must hold focus, so call `myscope.focus()` (often
  in `init`). The handler returns `accept`/`reject`:
  ```slint
  FocusScope {
      key-pressed(e) => {
          if (e.text == Key.Escape) { return accept; }
          if ((e.modifiers.control || e.modifiers.meta) && e.text == "a") { return accept; }
          return reject;
      }
  }
  ```
  Clicking a `TextInput`/widget steals focus; refocus the scope (e.g. on a
  background click) so shortcuts keep working.

## Overlays, Popovers & Context Menus

- `PopupWindow` (and the `ContextMenu`/`MenuBar` widgets) cover the common cases
  with auto-dismiss. For exact positioning (cursor-anchored menu, button-anchored
  dropdown), use a manual overlay:
  - Render it as a child of the top-level `Window` (so `x`/`y` are window coords),
    gated by `if open : …`.
  - Add a full-window backdrop `TouchArea` behind it to close on click.
  - Anchor with `widget.absolute-position.x/.y` (+ height); clamp with
    `Math.min(x, root.width - menu.width - 8px)` to stay on screen.
- A custom popover panel (a `Rectangle` directly under `Window`) defaults to
  filling the window — set `height: layout.preferred-height;` so it sizes to its
  content (see fill-vs-preferred in `reference/language-and-layout.md`).
