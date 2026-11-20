---
date: 2026-11-16
topic: Docker tag check — rust:1.98.1-slim vs rust:1.98-slim, and Clippy lint-set stability in 1.98.1
status: verified
---

# Docker tag check: `rust:1.98-slim` → `rust:1.98.1-slim`

## Question

1. Do the Docker Official Image tags `rust:1.98.1-slim` and `rust:1.98-slim` exist?
2. Is bumping `Dockerfile:1` (`FROM rust:1.98-slim AS builder`) to `rust:1.98.1-slim` safe?
3. Does Rust 1.98.1 change Clippy lint sets relative to 1.98.0?

## Findings

### 1. Both tags exist (Docker Hub, official `rust` image)

The Docker Hub tags listing for `rust` confirms all of the following are live official tags:

- `rust:1.98.1-slim` (plus `-slim-trixie`, `-slim-bookworm` variants)
- `rust:1.98-slim` (plus `-slim-trixie`, `-slim-bookworm` variants)
- `rust:1.98.0-slim` (older, still present)

**Crucially, `rust:1.98-slim` and `rust:1.98.1-slim` resolve to the identical digest** — e.g. linux/amd64 `fce83543909c...`, linux/386 `6987201413bf...`, linux/arm/v7 `d08db77881d8...`. The existing `1.98-slim` tag has already been re-pointed at the 1.98.1 image. The `1.98.0-slim` tag retains the distinct older digest (`90fd7674d9f6...` amd64).

### 2. The bump is safe — effectively a no-op

Since `rust:1.98-slim` already points to the same image as `rust:1.98.1-slim`, editing line 1 to the explicit patch tag changes no bytes pulled and no toolchain behavior today; it only makes the intended patch version explicit and pins against future re-tagging. The only content difference between 1.98.0 and 1.98.1 is a single rustc miscompilation fix (see below), which does not affect lint behavior.

### 3. Clippy lint sets are unchanged in 1.98.1

- The Rust **1.98.1 release notes** list exactly one change: *"rustc: fix miscompilation in generating vtables"* (rust-lang/rust#161441). No language, library, lint, Cargo, or Clippy changes.
- The **rust-clippy CHANGELOG** has a single `Rust 1.98` section (released 2026-08-20) covering the whole 1.98 line. There is no separate 1.98.1 entry, and no new lints, lint moves, or deprecations between 1.98.0 and 1.98.1. All 1.98 lint-set changes (new lints `unnecessary_unwrap_unchecked`, `chunks_exact_to_as_chunks`, `by_ref_peekable_peek`, `with_capacity_zero`, `manual_isolate_lowest_one`, `for_unbounded_range`, `unused_async_trait_impl`; `empty_enums` moved pedantic→nursery; `from_iter_instead_of_collect` deprecated) shipped in 1.98.0. The next lint-set changes appear in Rust 1.99.
- Patch releases in this toolchain do not add lint-set changes; Clippy bugs are only backported as false-positive/ICE fixes when severe, and none are recorded for 1.98.1.

Therefore CI's strict clippy gate (`cargo clippy --workspace --all-targets -- -D warnings`) will see the same lint set under 1.98.1 as under 1.98.0.

## Caveat

The pinned toolchain is Rust `1.98.0` (`rust-toolchain.toml` / CI), which pins the *compiler* used by local `cargo` invocations. The Dockerfile image is a separate toolchain source. Bumping the image to 1.98.1 does not alter the pinned local toolchain; if the intent is full version alignment, consider whether the pin should also move — out of scope for this tag check (no edits made).

## Primary sources

- Docker Hub official `rust` tags: https://hub.docker.com/_/rust/tags?name=1.98
- Rust 1.98.1 release notes: https://raw.githubusercontent.com/rust-lang/rust/1.98.1/RELEASES.md
- rust-clippy CHANGELOG: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md
- Issue #161441 (vtable miscompilation): https://github.com/rust-lang/rust/issues/161441
