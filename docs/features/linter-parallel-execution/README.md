# Linter Parallel Execution Feature

This folder contains documentation for the parallel execution optimization feature for linters.

## 📄 Documents

### [specification.md](./specification.md)

Complete specification for running linters in parallel including:

- Performance analysis and expected improvements
- Safety considerations (file conflicts)
- Implementation challenges (output handling)
- Cost-benefit analysis
- Integration with auto-fix feature
- Decision to defer implementation

### [questions.md](./questions.md)

Clarifying questions for implementation (if feature is prioritized in the future):

- Performance requirements and justification
- Output handling strategy options
- Grouping strategy for linters
- Async runtime choice (tokio, async-std, rayon)
- Refactoring scope and timeline
- Compatibility with auto-fix feature
- Testing strategy
- Error handling approach
- Configuration options
- Priority reassessment criteria

## 📊 Summary

**Goal**: Run linters in parallel to improve performance (~46% faster, 13s → 7s)

**Status**: **Deferred** - Not a priority for initial implementation

**Reason**: Current performance (15s) is acceptable for pre-commit workflow. Implementation complexity outweighs minimal performance gains.

**Reconsider when**: Execution time exceeds **25 seconds** (more linters added) or auto-fix feature makes it too slow.

## ✅ Alternative: Process-Level Parallelization (Already Possible)

**Discovery**: The linter binary already supports running individual linter types via command-line arguments, which enables **process-level parallelization** without code changes.

**Script Location**: `scripts/lint-parallel.sh`

**How it works**:

```bash
# Run individual linters in parallel processes
./target/release/linter markdown &
./target/release/linter yaml &
./target/release/linter toml &
./target/release/linter shellcheck &
./target/release/linter rustfmt &
wait

# Run clippy sequentially (may conflict with rustfmt)
./target/release/linter clippy

# Run cspell separately (read-only)
./target/release/linter cspell
```

**Performance**: ~14s (vs 15s sequential) - minimal improvement due to clippy dominating execution time (~12s)

**Trade-offs**:

- ✅ No code changes required
- ✅ Simple shell script implementation
- ✅ Easy to adjust grouping strategy
- ❌ Output may be interleaved (less readable)
- ❌ Minimal performance gain (~1s) since clippy dominates
- ❌ Less control over error aggregation and reporting

**Recommendation**: Use sequential execution (`cargo run --bin linter all`) for clean output. Process-level parallelization provides minimal benefit and potentially confusing output.

**Reconsider when**: Execution time exceeds **25 seconds** (more linters added) or auto-fix feature makes it too slow.

## 🎯 Key Insights

### Why Parallel Execution is Safe

Different linters modify different file types:

- markdown → `*.md`
- yaml → `*.yml`, `*.yaml`
- toml → `*.toml`
- rustfmt → `*.rs`
- shellcheck → `*.sh` (read-only)
- cspell → all files (read-only)

**Updated Strategy**: rustfmt can run in parallel with other linters. Only clippy needs to run sequentially after rustfmt if auto-fix is enabled.

### Why It's Complex

Current linters print errors **immediately** using `println!()`. Parallel execution would require:

1. Refactoring all 7 linters to capture output
2. Buffering results in memory
3. Displaying sequentially after parallel execution completes

This is non-trivial work for a 4-second improvement.

## 🔗 Related Features

- [Linter Auto-fix Feature](../linter-auto-fix/README.md) - Primary focus, higher priority
- Parallel execution can be added later if auto-fix is implemented and performance becomes an issue

## 📅 When to Reconsider

Consider implementing when:

- **Execution time exceeds 25 seconds** (trigger threshold)
- More linters are added (makes parallelization more valuable)
- Auto-fix feature makes linting too slow
- CI/CD performance becomes critical
- Auto-fix feature is stable and working well

## 🎯 Key Decisions (Based on Answered Questions)

1. **Performance threshold**: Reconsider when execution time > 25 seconds
2. **Output handling**: Use synchronized output mechanism (Option B)
3. **Grouping**: Group 1 (parallel): markdown, yaml, toml, shellcheck, rustfmt; Group 2 (sequential): clippy; cspell (separate group)
4. **Runtime**: Use **tokio** for async execution
5. **Refactoring**: Incremental approach - one linter at a time
6. **Auto-fix**: Must support from the start (auto-fix feature implemented first)
7. **Configuration**: Parallel by default, `--sequential` flag for debugging
8. **Timeline**: One day implementation once prioritized

## 🎯 Current Decision

**Focus on auto-fix feature first** (higher value). Parallel execution is deferred until:

- Auto-fix feature is complete and stable
- Execution time becomes problematic (>25 seconds)
- More linters are added making parallel execution more valuable
