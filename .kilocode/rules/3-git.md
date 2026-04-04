# Git Conventions

**Summary**: Conventional Commits specification for commit messages.

**Why**: Enables automated versioning and changelog generation.

---

## Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

---

## Commit Types

| Type | Description | Version |
|------|-------------|---------|
| `feat` | New feature | MINOR |
| `fix` | Bug fix | PATCH |
| `BREAKING CHANGE` | Breaking change | MAJOR |

**Additional**: `build:`, `chore:`, `ci:`, `docs:`, `style:`, `refactor:`, `perf:`, `test:`

---

## Scope

```
feat(ui): add word list component
fix(core): prevent racing in request handler
```

---

## Breaking Changes

Option 1 - `!` before colon:
```
feat(api)!: send email when product is shipped
```

Option 2 - Footer:
```
feat: allow config to extend other configs

BREAKING CHANGE: `extends` key is now used for extending other configs
```

---

## Rules

1. Type MUST be lowercase
2. Description REQUIRED after colon and space
3. Use imperative mood: "add feature" not "added feature"
4. Never use `git commit --amend` after pre-commit hook failure - create new commit instead

---

## Examples

```
docs: correct spelling of CHANGELOG
feat(lang): add Polish language
fix: prevent racing of requests

Introduce a request id and a reference to latest request.

Refs: #123
```
