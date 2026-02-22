# Git Commit Conventions

**Summary**: Follow Conventional Commits specification for commit messages.

**Why**: Provides consistent commit message format that enables automated versioning and changelog generation.

---

## Format

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

---

## Commit Types

| Type | Description | Semantic Versioning |
|------|-------------|---------------------|
| `feat` | A new feature | MINOR |
| `fix` | A bug fix | PATCH |
| `BREAKING CHANGE` | A breaking change | MAJOR |

### Additional Allowed Types

These types are allowed but don't affect semantic versioning unless they include a breaking change:

- `build:` - Changes that affect the build system
- `chore:` - Maintenance tasks
- `ci:` - Changes to CI configuration
- `docs:` - Documentation changes
- `style:` - Formatting, missing semicolons, etc.
- `refactor:` - Code restructuring without behavior changes
- `perf:` - Performance improvements
- `test:` - Adding or modifying tests

---

## Scope

A scope may be provided after the type to indicate the area of change:

```
feat(ui): add word list component
fix(core): prevent racing in request handler
docs(clinerules): update commit conventions
```

---

## Breaking Changes

### Option 1: Use `!` before the colon

```
feat(api)!: send email when product is shipped
```

### Option 2: Footer notation

```
feat: allow config to extend other configs

BREAKING CHANGE: `extends` key is now used for extending other configs
```

---

## Examples

### Simple commit

```
docs: correct spelling of CHANGELOG
```

### With scope

```
feat(lang): add Polish language
```

### With body and footer

```
fix: prevent racing of requests

Introduce a request id and a reference to latest request. Dismiss
incoming responses other than from latest request.

Remove timeouts which were used to mitigate the racing issue.

Refs: #123
Reviewed-by: Z
```

### Breaking change with `!`

```
feat!: remove deprecated API endpoint

BREAKING CHANGE: the /api/v1/words endpoint has been removed.
Use /api/v2/words instead.
```

---

## Rules

1. Type MUST be lowercase
2. Description is REQUIRED and follows the colon and space
3. Body is optional, begins one blank line after description
4. Footers follow the body, one blank line after
5. Breaking changes are case-insensitive except for `BREAKING CHANGE` token
6. Use imperative mood: "add feature" not "added feature"
