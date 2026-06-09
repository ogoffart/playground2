# Slint × Agentic Coding — proposed improvements

Ready-to-file issue drafts for [`slint-ui/slint`](https://github.com/slint-ui/slint),
ordered by impact on the **AI-agent coding experience** (highest first). Each `.md`
is a self-contained issue you can paste into the tracker.

These came out of building a non-trivial Slint app (a desktop file manager) end to
end with a coding agent. Every item maps to concrete friction hit during that
session, not speculation.

## Priority order

1. **Machine-readable docs for agents** — [`issue-agent-docs-llms-txt.md`](issue-agent-docs-llms-txt.md)
   `llms.txt` + a compact language/builtins reference. *Broadest impact:* stops
   agents guessing APIs or reading the compiler source, preventing wrong code at
   the source.

2. **Headless `slint check` with machine-readable diagnostics** — [`issue-agent-headless-check-cli.md`](issue-agent-headless-check-cli.md)
   A one-shot, LSP-backed CLI so any agent (no IDE/LSP wiring, no full `cargo
   build`) gets `.slint` diagnostics in milliseconds. Tightens the core
   write→check→fix loop.

3. **Headless MCP rendering (screenshots)** — [`slint-mcp-headless-rendering-issue.md`](slint-mcp-headless-rendering-issue.md)
   Make `--features slint/mcp` render with no display/Xvfb so agents can verify the
   *real running UI*. (Includes the `backend-testing` re-export ask.)

4. **Ship the agent skill out of the box** — [`issue-agent-skill-distribution.md`](issue-agent-skill-distribution.md)
   Plugin + marketplace + template bundling so users' agents actually get Slint
   guidance. Today the skill exists in the repo but is never auto-discovered.

5. **Compiler/LSP diagnostics tuned for agents** — [`issue-compiler-diagnostics.md`](issue-compiler-diagnostics.md)
   "did-you-mean" suggestions, structural hints, and stable JSON diagnostics to cut
   agent retry loops.

6. **Agent-ready project templates** — [`issue-agent-templates.md`](issue-agent-templates.md)
   Scaffolds that encode the idioms agents otherwise reinvent (globals interop,
   models, theming) and bundle the skill/MCP config.

## Themes

- **Feedback loop** (#2, #3, #5): make "did my change work?" fast and headless.
- **Knowledge delivery** (#1, #4, #6): get correct, current Slint knowledge in
  front of the agent without it guessing.
