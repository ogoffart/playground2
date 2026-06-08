# The `.slint` Language & Layout

## Language Essentials

Declarative and reactive: a property binding re-evaluates automatically when
anything it reads changes.

```slint
import { Button, VerticalBox } from "std-widgets.slint";

// Root element decides the component's nature: a Rectangle fills its parent;
// a layout sizes to its content (see Layout & Sizing).
component Counter inherits Rectangle {
    in property <string> label;          // set by parent / host language
    out property <int> count;            // readable by the parent
    in-out property <bool> enabled: true;// read+write both sides
    private property <int> step: 1;      // internal only
    callback changed(int);

    background: area.has-hover ? #eee : transparent;  // reactive

    VerticalBox {
        Text { text: "\{root.label): \{root.count)"; }   // interpolation
        Button { text: "+"; clicked => { root.count += root.step; root.changed(root.count); } }
    }
    area := TouchArea { }                 // `name :=` assigns an id
}
```

- **Property directions**: `in` (parent/host writes), `out` (component writes),
  `in-out`, `private`. Be explicit for anything crossing a boundary.
- **Two-way binding**: `width <=> other.width;`.
- **Callbacks**: `callback foo(int);` then `foo => { ... }` or `self.foo(1)`; may
  return values.
- **Control flow**: `if cond : Elem { }`; `for item[index] in model : Elem { }`;
  `for i in 5 :` iterates `0..4`.
- **`@children`**: forward injected children into a placeholder.
- **Globals**: `export global Foo { ... }` — shared state, theme tokens, host
  interop (see `reference/interop.md`).
- **`init => { ... }`**: runs once on creation (e.g. `some-focus-scope.focus()`).

## Layout & Sizing (read before fighting the layout)

- Use layouts (`VerticalLayout`, `HorizontalLayout`, `GridLayout`, or the padded
  `*Box` widgets); reserve `x`/`y` for overlays, popovers, and custom drawing.
- **`padding`/`spacing` only work on layout elements.** On a `Text`/`Rectangle`
  they're silently ignored (the compiler warns). To inset a `Text`, wrap it:
  `HorizontalLayout { padding-left: 6px; Text {...} }`.
- **Fill vs. preferred size.** `Rectangle`, `TouchArea`, `FocusScope`, `Flickable`
  fill their parent by default; `Text`, `Image`, `Path`, and layouts take their
  *preferred* size. **A custom component or a layout placed inside a non-layout
  parent (a `Rectangle`, or the `Window`) does NOT stretch — it sits at preferred
  size, often centered.** Fix with `width: 100%; height: 100%`, by making the
  parent a layout, or by making the layout the component's root.
- Distribute space with `horizontal-stretch`/`vertical-stretch` and
  `min/preferred/max-width/height`; a stretched `Rectangle { }` is a flexible
  spacer.
- Z-order: later siblings render on top.
- `Window` (or `Dialog`) is the only valid top-level exported element for an app.
