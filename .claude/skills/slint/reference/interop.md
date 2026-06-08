# Connecting to Business Logic (Rust / C++ / JS / Python)

The cleanest way to wire a non-trivial app is **two globals**: one carrying data
*into* the UI, one carrying callbacks *out*. This avoids threading dozens of
properties/callbacks through every component.

```slint
// globals.slint
export struct Row { id: int, name: string, tags: [int], selected: bool }
export global AppData {           // host pushes models/state in
    in property <[Row]> rows;
    in property <string> status;
}
export global Logic {            // UI calls these; host handles them
    callback row-clicked(int, bool, bool);   // index, ctrl, shift
    callback refresh();
}
```

Any component can read `AppData.rows` / call `Logic.row-clicked(...)` directly.

## Rust

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
  `row-clicked` → callback registrar `on_row_clicked` / setter `set_...`.
- `[T]` array properties become `ModelRc<T>`; build with
  `ModelRc::new(VecModel::from(vec))`. For live updates, keep a `Rc<VecModel<T>>`
  and mutate it (`push`/`set_row_data`) instead of replacing the whole model.
- `string` ↔ `SharedString` (`.into()` from `String`/`&str`); `length`/`float` ↔
  `f32`; `int` ↔ `i32`; `brush`/`color` ↔ `slint::Brush`/`Color`.
- A common split: keep the source of truth and all logic in the host language,
  expose the current view as an already-sorted/filtered model, and let `.slint`
  render + forward interactions. Replace prototype timers with real signals
  (`slint::Timer` for periodic UI work; from other threads use
  `slint::invoke_from_event_loop` with a `Weak` handle).

## C++ / JS / Python

The same globals/structs are exposed through each language's idiomatic API
(getters/setters and callback registration); property/callback names follow each
language's conventions (e.g. snake_case in Python, camelCase in JS). The
two-globals pattern and the "logic in host, view model into `.slint`" split apply
identically.
