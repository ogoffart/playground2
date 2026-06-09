# Machine-readable docs for AI agents (`llms.txt` + compact language/API reference)

## Summary

Coding agents building Slint apps can't efficiently consume the HTML docs, so they
guess at the API or fall back to reading the compiler source. Provide an
agent-consumable form of the docs:

1. **`llms.txt` / `llms-full.txt`** at `slint.dev` (and per version at
   `releases.slint.dev/<version>/`), following the emerging convention.
2. **A single compact, authoritative reference** of the language + builtin
   elements/properties/types + color/math functions, generated from the same
   source as the reference docs.

## Motivation (from a real agent session)

Building a file-manager app, the agent repeatedly had to `grep` the Slint source to
confirm things that are documented-or-derivable but not agent-consumable:

- `Path` `stroke-width` is **not** scaled by the viewbox (must scale manually).
- `oklch()` / `hsv()` color functions exist — the agent initially **assumed they
  did not** and converted design tokens to hex unnecessarily.
- `TouchArea.pointer-event` fields (`kind`/`button`/`modifiers`) for modifier-aware
  clicks.
- `FocusScope.key-pressed` returning `accept`/`reject`, and `forward-focus`.
- The fill-vs-preferred sizing rule (why a layout/custom component doesn't fill).

Each wrong assumption became a failed compile or a source-reading detour — wasted
wall-clock and tokens. An agent that can fetch one authoritative, token-efficient
file avoids the whole class of error.

## Proposal

- Publish `llms.txt` (curated index of key pages) and `llms-full.txt` (concatenated
  docs) at `https://slint.dev/llms.txt` and per-version under `releases.slint.dev`.
- Generate a **compact "language + builtins" reference**: every builtin element
  with its properties and types, builtin functions, enums, and the color/math
  helpers — one fetchable document.
- Ensure doc pages are retrievable as **plain Markdown** (not JS-only rendered) so
  agent web-fetch tools work.

## Acceptance criteria

- [ ] An agent can fetch one or two stable URLs and get current, complete,
      token-efficient coverage of the language + element/property API for a given
      Slint version.
- [ ] Available per released version, not just latest.

## Pointers

- Builtins: `internal/compiler/builtins.slint`; widgets:
  `internal/compiler/widgets/`.
- Existing docs site generation under `docs/`.
