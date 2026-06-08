# The `.slint` Language & Layout

## Language Essentials

`.slint` files are declarative and reactive: a property binding is an expression
that is automatically re-evaluated when anything it reads changes.

```slint
import { Button, VerticalBox } from "std-widgets.slint";

// A reusable component. The root element decides the component's nature
// (a Rectangle fills its parent; a layout sizes to its content — see Sizing).
component Counter inherits Rectangle {
    in property <string> label;          // set by the parent / host language
    out property <int> count;            // readable by the parent
    in-out property <bool> enabled: true;// read+write both sides
    private property <int> step: 1;      // internal only
    callback changed(int);               // notify the outside world

    background: area.has-hover ? #eee : transparent;  // reactive binding

    VerticalBox {
        Text { text: "\{root.label): \{root.count)"; }   // string interpolation
        Button {
            text: "+";
            clicked => { root.count += root.step; root.changed(root.count); }
        }
    }
    area := TouchArea { }                 // `name :=` gives an element an id
}
```

Key constructs:
- **Property directions**: `in` (parent/host writes), `out` (component writes),
  `in-out` (both), `private` (internal). Be explicit for anything crossing a
  boundary.
- **Two-way binding**: `width <=> other.width;` keeps two properties in sync.
- **Callbacks**: `callback foo(int, string);` then `foo => { ... }` or
  `self.foo(1, "x")`. Callbacks may return values and have a body.
- **Conditionals & loops**: `if cond : Elem { }` and
  `for item[index] in model : Elem { }`. `for i in 5 : ...` iterates `0..4`.
- **`@children`**: forward injected children into a placeholder inside a component.
- **Globals**: `export global Foo { ... }` — singletons for shared state, theme
  tokens, and the host-language interop surface (see `reference/interop.md`).
- **`init => { ... }`** runs once when an element is created (handy to call
  `some-focus-scope.focus()`).

## Layout & Sizing (read this before fighting the layout)

- Put elements in layouts (`VerticalLayout`, `HorizontalLayout`, `GridLayout`, or
  the padded `*Box` widgets) instead of positioning by hand. Use `x`/`y` only for
  overlays, popovers, and custom-drawn content.
- **`padding` and `spacing` only do something on layout elements.** Setting
  `padding-left` on a `Text` or `Rectangle` is silently ignored (the compiler
  warns). To inset a `Text`, wrap it:
  `HorizontalLayout { padding-left: 6px; Text {...} }`.
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
- Z-order: later siblings render on top of earlier ones.
- `Window` (and `Dialog`) is the only valid top-level exported element for an
  application entry point.
