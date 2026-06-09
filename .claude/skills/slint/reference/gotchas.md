# Gotchas & Common Compile Errors

These bite almost everyone at least once.

- **Units.** `length` (`px`, `pt`, `rem`, …) and `int`/`float` are distinct types.
  Convert with `value * 1px` or `len / 1px`. `length`-typed properties like
  `letter-spacing` reject unitless/`em` values — use `px` (e.g. `0.4px`, not
  `0.04em`).
- **Colors.** Use hex literals (`#rgb`, `#rrggbb`, `#rrggbbaa`) or the color
  functions: `rgb(r,g,b)`, `rgba(r,g,b,a)` (alpha `0.0..1.0`), `hsv(h,s,v[,a])`,
  and **`oklch(l, c, h[, a])`** — so OKLCH design tokens can be used directly,
  e.g. `oklch(0.55, 0.17, 256)`. In `oklch`, `l` is lightness `0..1`, `c` is
  chroma (a number, or a `%` where `100% == 0.4`), `h` is hue in degrees (a number
  or an `angle`), `a` is alpha `0..1`. (There is `hsv` but no `hsl`.) Convert a
  color back with `.to-oklch()` / `.to-hsv()`. Color helpers: `.brighter(f)`,
  `.darker(f)`, `.with-alpha(a)`, `.transparentize(f)`, `.mix(other, f)`; read
  channels with `.red`/`.green`/`.blue`/`.alpha`.
- **Math functions** come in two callable forms (not bare names, generally):
  - methods on a number: `x.floor()`, `x.ceil()`, `x.round()`, `x.sqrt()`,
    `x.mod(y)`, `x.abs()`, `x.clamp(lo, hi)`, `x.max(y)`, `x.min(y)`,
    `x.to-fixed(2)` (→ string), `x.to-precision(3)`;
  - the `Math` namespace: `Math.max(a, b)`, `Math.min`, `Math.clamp`,
    `Math.round`, `Math.floor`, `Math.pow`, `Math.sin`, `Math.atan2`, …
  Use these instead of guessing a bare `floor(...)`. Integer division yields a
  float, so wrap with `.floor()` when assigning to an `int`.
- **Rotation** is only on `Image` and `Text` — **not** on `Rectangle` or arbitrary
  components. Use `transform-rotation` (the `rotation-angle` alias is deprecated).
  To "rotate" something else, rotate an `Image`, swap to a pre-rotated glyph, or
  express the effect another way (a gradient for diagonal stripes, a flipped path
  for a chevron).
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
- **`padding`/`spacing` are ignored on non-layout elements** — see
  `reference/language-and-layout.md`.

For "element won't fill / is centered" and "padding does nothing", see the Layout
& Sizing section in `reference/language-and-layout.md`.
