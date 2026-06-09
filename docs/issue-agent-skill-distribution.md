# Distribute the Slint "agent skill" so users get it out of the box

## Summary

Slint ships an agent skill at `skills/slint/SKILL.md`, but coding agents don't
auto-discover it — not from the repo, and not from the published crate/npm/wheel
(agents don't scan dependency trees). Deliver it through channels agents actually
read:

1. a **Claude Code plugin + marketplace** hosted in this repo,
2. **bundling** `.claude/skills/slint/` (+ `.mcp.json`, `AGENTS.md`) in the
   official project templates,
3. keeping `SKILL.md` **vendor-neutral** (the Agent Skills standard).

## Motivation (from a real agent session)

The skill was physically present in the git-checked-out dependency the entire
session (`~/.cargo/git/checkouts/slint-*/skills/slint/SKILL.md`, because the app
used a git dependency) and was **never used until a human pasted its URL**.
Discovery — not content — is the gap.

Verified discovery rules (Claude Code): skills are loaded only from
`~/.claude/skills/`, `<project>/.claude/skills/`, and **enabled plugins**. Nothing
scans `node_modules`, `~/.cargo`, or `site-packages` — so shipping the skill inside
the crate/npm/wheel is invisible (pure download cost, zero discovery).

## Proposal

### 1. Plugin + marketplace in `slint-ui/slint`

Plugins auto-discover skills under `<plugin-root>/skills/`, and a marketplace can
live in the same repo, so the existing `skills/slint/` becomes installable by
adding two manifests at the repo root:

```
.claude-plugin/plugin.json        # { "name": "slint", "description": "...", "version": "..." }
.claude-plugin/marketplace.json   # lists the plugin, source "."
skills/slint/                     # already here → auto-included
```

Users then run, once, and get it in every project:

```
/plugin marketplace add slint-ui/slint
/plugin install slint@slint
```

Bundle the runtime **MCP config** (`.mcp.json` → `http://localhost:9315/mcp`) and a
`/slint:screenshot` command in the same plugin.

### 2. Bake it into the official templates

Add `.claude/skills/slint/` + `.mcp.json` + a short `AGENTS.md` pointer to the
`create-slint-app` / cargo-generate / C++ / Python starters, so **new projects are
agent-ready with zero steps** (the highest-leverage path for new users).

### 3. Keep `SKILL.md` portable

Stick to the [Agent Skills standard](https://agentskills.io) (`name`/`description`
+ Markdown, no tool-specific frontmatter) so Cursor, Codex CLI, Gemini CLI, and
Copilot benefit from the same file.

## Acceptance criteria

- [ ] A documented one-line install makes the skill available across a user's
      projects.
- [ ] Newly scaffolded template projects include the skill automatically.
- [ ] `SKILL.md` remains usable by non-Claude agents.
