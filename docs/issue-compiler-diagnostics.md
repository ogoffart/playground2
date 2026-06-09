# Tune compiler/LSP diagnostics for agents (suggestions + machine-readable output)

## Summary

Confusing or terse diagnostics cause agent **retry loops** (each = latency +
tokens). Slint's messages are already good in places; extend that with
"did-you-mean" suggestions, hints for common structural traps, and stable
machine-readable diagnostics.

## Motivation (from a real agent session)

What the agent hit, and how costly each was to recover from:

- `letter-spacing: 1em` → **great** message: *"Invalid unit 'em'. Valid units
  are: …"*. Fixed immediately. (This is the bar to hit everywhere.)
- bare math calls — the agent guessed `floor(x)`; the working forms are `x.floor()`
  / `Math.floor(x)`. A suggestion would have saved a cycle.
- `padding`/`spacing` on non-layout elements → a warning exists (good), but the
  fix ("wrap in a layout") isn't hinted.
- a layout / custom component **silently sitting at preferred size** inside a
  non-layout parent (the "won't fill" trap) → no diagnostic at all; this is a
  frequent confusion.
- `rotation-angle` on a `Rectangle` → clear *"Unknown property"* (good); the
  deprecation of `rotation-angle` → `transform-rotation` is also surfaced (good).

## Proposal

- **"did you mean"** for unknown properties, elements, and enum values (nearest
  match within the element's known set).
- A **hint when a bare math function** is used (suggest `.floor()` / `Math.floor`).
- An optional **hint for the "won't fill" trap**: when a layout or custom component
  is a child of a non-layout element with no explicit size, suggest
  `width/height: 100%` or wrapping in a layout.
- **Stable JSON diagnostics** from the compiler (and via a `slint check` CLI — see
  the headless-check issue) so agent harnesses can parse and act on them.

## Acceptance criteria

- [ ] Common mistakes produce an actionable suggestion, not just a rejection.
- [ ] Diagnostics are available as documented, stable JSON.

## Pointers

- Diagnostic engine: `internal/compiler` (diagnostics, type resolution,
  `lookup.rs` for known properties/members — a source for "did you mean" candidate
  sets).
