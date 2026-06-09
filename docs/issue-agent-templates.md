# Agent-ready project templates / scaffolding

## Summary

Provide official starters that **encode the idioms agents otherwise reinvent** and
**bundle the agent tooling**, so `create-slint-app` (and the cargo-generate / C++ /
Python templates) yield a project an agent can immediately build, run, screenshot,
and extend correctly.

## Motivation (from a real agent session)

Starting from an empty repo, the agent had to:

- hand-write `Cargo.toml`, `build.rs`, and `slint::include_modules!()` wiring;
- **invent** an interop architecture — it landed on a two-globals split
  (`AppData` for data-in, `Logic` for callbacks-out) and a `Theme` global for
  light/dark — patterns a template should provide rather than leave to chance;
- set up models (`ModelRc`/`VecModel`) and a Rust↔Slint naming/type mapping from
  scratch.

Templates double as ground truth: agents copy the patterns they see.

## Proposal

Ship starters that include:

- **Build wiring** (Rust/C++/JS/Python) ready to run.
- A **globals-based interop sample**: data-in / callbacks-out, with a `ModelRc`
  example and the kebab→snake / type-mapping conventions shown.
- A **`Theme` global** using `Palette` for light/dark and `Palette.accent-background`
  for the OS accent.
- **Obvious scripts** (e.g. a `justfile`): `run`, `check` (headless diagnostics),
  `screenshot` (via `slint-viewer --screenshot`).
- **Agent config**: `.claude/skills/slint/` (or the plugin), `.mcp.json`, and a
  short `AGENTS.md`.

A `cargo slint new` / `npm create slint` could scaffold all of the above.

## Acceptance criteria

- [ ] A freshly scaffolded project **builds, runs, and screenshots headlessly**.
- [ ] It demonstrates the recommended interop, theming, and model patterns.
- [ ] It is agent-ready (skill + MCP config present) with no extra setup.

## Related

- Skill distribution: `issue-agent-skill-distribution.md`
- Headless screenshots/diagnostics used by the scripts:
  `slint-mcp-headless-rendering-issue.md`, `issue-agent-headless-check-cli.md`
