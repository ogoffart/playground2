# Add a headless `slint check` (LSP-backed) with machine-readable diagnostics

## Summary

Agents without IDE/LSP integration have no fast way to get `.slint` diagnostics
short of a full `cargo build`. The language server already computes exactly this.
Expose a one-shot CLI:

```sh
slint check ui/main.slint            # human-readable; non-zero exit on error
slint check --format json ui/*.slint # machine-readable for tooling
```

## Motivation (from a real agent session)

While building a file-manager app, the agent's only static-feedback signals were
`cargo build` (the **first build compiled Slint from source for minutes**) and
grepping the source. To use the LSP headlessly, it had to **hand-roll a JSON-RPC
stdio client** (`initialize` → `initialized` → `textDocument/didOpen` → collect
`textDocument/publishDiagnostics`).

That client *did* work against `slint-lsp 1.17.0`, fully headless, and was fast and
accurate:

- `ui/theme.slint`, `ui/main.slint` → `0 diagnostics` in well under a second (no
  `cargo build`).
- `letter-spacing: 1em` → `error: Invalid unit 'em'. Valid units are: %, phx, px,
  cm, mm, in, pt, rem, s, ms, deg, grad, turn, rad`.
- `rotation-angle` → `warning: deprecated, use 'transform-rotation'`;
  `rotation-angle` on a `Rectangle` → `error: Unknown property`.

But every agent harness shouldn't have to reimplement an LSP client. A plain CLI
makes headless diagnostics trivial and harness-agnostic, and is mostly a matter of
surfacing the existing compiler/LSP diagnostics through argv + stdout.

## Proposal

- Add `slint check <files...>` (or `slint-lsp --check`): load the files (resolving
  imports, `--style`, `-I`/`-L` like the viewer/build), run the compiler's
  diagnostic pass, print human-readable output by default and `--format json` for
  tooling. Exit code reflects whether there were errors.
- Keep the JSON schema stable (path, range, severity, message, code).

## Acceptance criteria

- [ ] `slint check ui/main.slint` returns diagnostics + a meaningful exit code with
      no `cargo build` and no running app.
- [ ] `--format json` output is documented and stable for agent consumption.
- [ ] Import/style resolution matches the compiler/viewer.

## Notes

This complements (does not replace) the MCP server: `check` is *static* analysis
(fast, every keystroke-equivalent); the MCP server is *runtime* inspection of a
live app.

## Pointers

- `tools/lsp/` (diagnostics already computed here), `internal/compiler` (the
  diagnostic engine), `tools/viewer` (precedent for a `.slint`-loading CLI with
  `-I`/`-L`/`--style`).
