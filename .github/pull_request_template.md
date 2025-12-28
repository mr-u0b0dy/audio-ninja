# Pull Request

## Description
<!-- Describe the changes in this PR -->

## Type of Change
<!-- Mark the relevant option with an "x" -->

- [ ] 🐛 Bug fix (non-breaking change that fixes an issue)
- [ ] ✨ Feature (non-breaking change that adds functionality)
- [ ] 📚 Documentation (documentation or examples)
- [ ] 🎨 Style (formatting, naming, code cleanup)
- [ ] ♻️ Refactoring (code restructuring without behavior change)
- [ ] 🚀 Performance (performance improvement)
- [ ] 🔧 CI/CD (workflow, build, or dependency updates)
- [ ] ⚠️ Breaking change (breaking change that requires version bump)

## Related Issues
<!-- Link to related issues, e.g., "Fixes #123" -->

## Changes Made
<!-- Bullet list of specific changes -->

- 
- 
- 

## Testing
<!-- Describe testing performed -->

- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed
- [ ] Code coverage maintained or improved

## Testing Instructions
<!-- How can reviewers test this change? -->

```bash
# Example commands to test the change

```

## Checklist

### Code Quality
- [ ] Code follows [Rust 2021 style guidelines](https://rust-lang.github.io/api-guidelines/)
- [ ] `cargo fmt` applied
- [ ] `cargo clippy` passes (no warnings treated as errors)
- [ ] New public APIs documented with doc comments
- [ ] No unnecessary dependencies added

### Testing
- [ ] All existing tests pass: `cargo test --workspace --all-features`
- [ ] Benchmarks checked if performance-sensitive: `cargo bench`
- [ ] No panics or unwrap() on untrusted input
- [ ] Error handling is appropriate

### Documentation
- [ ] README.md updated if needed
- [ ] API documentation updated if public APIs changed
- [ ] Examples added for new features
- [ ] Comments added for non-obvious logic

### Commits
- [ ] Commits are logical and atomic
- [ ] Commit messages follow convention: `type: description`
- [ ] No large auto-generated files included

### Review
- [ ] Self-review completed
- [ ] Code is ready for review

## Performance Impact
<!-- Any expected performance impact? -->

- [ ] No performance impact
- [ ] Improves performance
- [ ] Minor regression (acceptable because...)
- [ ] Performance not applicable

## Rollback Plan
<!-- How would this change be rolled back if needed? -->

## Additional Notes
<!-- Any additional context or concerns -->

## Screenshots/Videos (if applicable)
<!-- For GUI changes, include screenshots -->

---

**Pre-submission Checklist:**
- [ ] I have read the [CONTRIBUTING.md](../CONTRIBUTING.md) guidelines
- [ ] I have searched for duplicate PRs
- [ ] I have tested this locally
- [ ] CI passes on this branch
- [ ] I agree to contribute this code under the Apache 2.0 license
