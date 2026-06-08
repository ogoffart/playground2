# Input Handling, Overlays & Menus

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
  `myscope.focus()` (often in `init`). The handler returns `accept`/`reject`:
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
  (`height: layout.preferred-height;`) so it sizes correctly. (See the fill-vs-
  preferred note in `reference/language-and-layout.md`.)
