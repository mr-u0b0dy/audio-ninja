# Git Hooks

This directory contains Git hooks for Audio Ninja.

## Installation

To enable these hooks, run:

```bash
git config core.hooksPath .githooks
```

Or use the setup script:

```bash
./scripts/dev-setup.sh
```

## Available Hooks

### pre-commit

Runs comprehensive validation before each commit:

1. **Format check**: Ensures code is formatted (`cargo fmt --check`)
2. **Clippy**: Lints with warnings as errors (`cargo clippy -- -D warnings`)
3. **Tests**: Runs all tests except E2E (`cargo test -- --skip e2e_`)
4. **Documentation**: Builds docs to verify they compile

If any check fails, the commit is aborted.

### Skipping Hooks

In rare cases where you need to bypass the pre-commit hook:

```bash
git commit --no-verify -m "message"
```

**Note**: This should be avoided. CI will still run all checks.

## VS Code Alternative

If you prefer running checks manually in VS Code:

1. Press `Ctrl+Shift+P`
2. Select "Tasks: Run Task"
3. Choose "Pre-Commit: Full Validation"

Or press `Ctrl+Shift+B` to run the default test task.
