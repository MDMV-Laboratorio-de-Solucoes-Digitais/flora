# Independent verification: Cargo test selection, `#[ignore]`, `--all-targets`, and GitHub Actions `run:` edits

- **Date / access date:** 2026-11-16
- **Method:** direct retrieval of official documentation only (The Cargo Book, The rustc Book, The Rust Reference, The Rust Programming Language book on `doc.rust-lang.org`; GitHub Actions docs on `docs.github.com`). No third-party sources, no inference from local code.
- **Scope:** four questions about `cargo test` semantics and one about GitHub Actions workflow edits.

---

## 1. Bare `cargo test` in a non-virtual workspace with a root package and no `default-members`

**Answer: only the root package (root crate) is selected — not all workspace members.**

Source A — *The Cargo Book* § `cargo-test(1)` → Package Selection,
URL: https://doc.rust-lang.org/cargo/commands/cargo-test.html#package-selection
(accessed 2026-11-16). Official text:

> By default, when no package selection options are given, the packages selected depend on the selected manifest file (based on the current working directory if `--manifest-path` is not given). If the manifest is the root of a workspace then the workspaces default members are selected, otherwise only the package defined by the manifest will be selected.
>
> The default members of a workspace can be set explicitly with the `workspace.default-members` key in the root manifest. If this is not set, a virtual workspace will include all workspace members (equivalent to passing `--workspace`), and a non-virtual workspace will include only the root crate itself.

Source B — *The Cargo Book* § Workspaces → `default-members`,
URL: https://doc.rust-lang.org/cargo/reference/workspaces.html#the-default-members-field
(accessed 2026-11-16). Official text:

> When unspecified, the root package will be used. In the case of a virtual workspace, all members will be used (as if `--workspace` were specified on the command-line).

For completeness, the same page's Package Selection section (https://doc.rust-lang.org/cargo/reference/workspaces.html#package-selection, accessed 2026-11-16) states:

> In a workspace, package-related Cargo commands like `cargo build` can use the `-p` / `--package` or `--workspace` command-line flags to determine which packages to operate on. If neither of those flags are specified, Cargo will use the package in the current working directory. However, if the current directory is a workspace root, the `default-members` will be used.

The `default-members` section also carries this note (quoted verbatim, same URL):

> When a root package is present, you can only operate on it using `--package` and `--workspace` flags.

**Conclusion:** A bare `cargo test` run from the workspace root of a *non-virtual* workspace (one whose root `Cargo.toml` has both `[workspace]` and `[package]`) with no `workspace.default-members` selects exactly one package: the root package. It is **not** equivalent to `--workspace` in this case; that equivalence holds only for a *virtual* manifest.

---

## 2. What `--workspace` does

**Answer: selects/operates on all members of the workspace.**

Source — *The Cargo Book* § `cargo-test(1)` → Package Selection,
URL: https://doc.rust-lang.org/cargo/commands/cargo-test.html#option-cargo-test---workspace
(accessed 2026-11-16). Official text:

> `--workspace`
> : Test all members in the workspace.
>
> `--all`
> : Deprecated alias for `--workspace`.

Related flag on the same page (https://doc.rust-lang.org/cargo/commands/cargo-test.html#option-cargo-test---exclude, accessed 2026-11-16):

> `--exclude` *SPEC*…
> : Exclude the specified packages. Must be used in conjunction with the `--workspace` flag.

**Conclusion:** `--workspace` overrides the default package selection and tests every workspace member; `--all` is only a deprecated alias. It does **not** imply an ignored-test or all-targets behavior — those are separate flags.

---

## 3. Do `#[ignore]` tests run by default?

**Answer: No. They are compiled but not executed by a default `cargo test`; `-- --ignored` runs only them, `-- --include-ignored` runs them together with the rest.**

Source A — *The Rust Reference* § Testing attributes → The `ignore` attribute,
URL: https://doc.rust-lang.org/reference/attributes/testing.html#the-ignore-attribute
(accessed 2026-11-16). Official text:

> The *`ignore` attribute* can be used with the `test` attribute to tell the test harness to not execute that function as a test.
>
> Ignored tests are still compiled when in test mode, but they are not executed.
>
> The `rustc` test harness supports the `--include-ignored` flag to force ignored tests to be run.

Source B — *The rustc Book* § Tests → Test attributes / Selection options,
URL: https://doc.rust-lang.org/rustc/tests/index.html#test-attributes and https://doc.rust-lang.org/rustc/tests/index.html#--ignored
(accessed 2026-11-16). Official text:

> `#[ignore]` — Indicates that the test function will be compiled, but not run by default. See the `--ignored` and `--include-ignored` options to run these tests.
>
> `--ignored`
> Runs only tests that are marked with the `ignore` attribute.
>
> `--include-ignored`
> Runs both ignored and non-ignored tests.

The same page also documents how these reach libtest (https://doc.rust-lang.org/rustc/tests/index.html#cli-arguments, accessed 2026-11-16):

> Note: When running with `cargo test`, the libtest CLI arguments must be passed after the `--` argument to differentiate between flags for Cargo and those for the harness. For example: `cargo test -- --no-capture`

Source C — *The Rust Programming Language* (official book) § Ignoring Tests Unless Specifically Requested,
URL: https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-tests-unless-specifically-requested
(accessed 2026-11-16). Official text:

> After `#[test]`, we add the `#[ignore]` line to the test we want to exclude. Now when we run our tests, `it_works` runs, but `expensive_test` doesn't:
>
> `test result: ok. 1 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s`
>
> If we want to run only the ignored tests, we can use `cargo test -- --ignored`:
>
> If you want to run all tests whether they're ignored or not, you can run `cargo test -- --include-ignored`.

**Conclusion:** `#[ignore]` tests are excluded from default runs (shown as `ignored`). Neither `--workspace` nor `--all-targets` changes that — they select packages/targets, not ignored-test status. To execute ignored tests, pass the libtest flags after `--`: `cargo test -- --ignored` or `cargo test -- --include-ignored` (add `-- --ignored` on top of any package/target selection).

---

## 4. What `--all-targets` includes

**Answer: exactly `--lib --bins --tests --benches --examples`. Notably, doc tests are not included.**

Source — *The Cargo Book* § `cargo-test(1)` → Target Selection,
URL: https://doc.rust-lang.org/cargo/commands/cargo-test.html#option-cargo-test---all-targets
(accessed 2026-11-16). Official text:

> `--all-targets`
> : Test all targets. This is equivalent to specifying `--lib --bins --tests --benches --examples`.
>
> `--doc`
> : Test only the library's documentation. This cannot be mixed with other target options.

Supporting context from the same page's Target Selection section (https://doc.rust-lang.org/cargo/commands/cargo-test.html#target-selection, accessed 2026-11-16):

> When no target selection options are given, `cargo test` will build the following targets of the selected packages:
>
> - lib — used to link with binaries, examples, integration tests, and doc tests
> - bins (only if integration tests are built and required features are available)
> - examples — to ensure they compile
> - lib as a unit test
> - bins as unit tests
> - integration tests
> - doc tests for the lib target
>
> Passing target selection flags will test only the specified targets.

From *The Cargo Book* § Cargo Targets → The `test` field (https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-test-field, accessed 2026-11-16):

> The `test` field indicates whether or not the target is tested by default by `cargo test`. The default is `true` for lib, bins, and tests.
>
> Examples are built by `cargo test` by default to ensure they continue to compile, but they are not *tested* by default. Setting `test = true` for an example will also build it as a test and run any `#[test]` functions defined in the example.

**Conclusion:** `--all-targets` = `--lib --bins --tests --benches --examples`. It is a *target*-selection flag, not a package- or ignore-selection flag. Because `--doc` is a separate and mutually exclusive target option, doc tests are **not** part of `--all-targets`; the default (no target flags) run is the only one that includes doc tests for the lib target. If doc tests must also run, they need a separate `cargo test --doc` invocation (e.g. `cargo test --all-targets && cargo test --doc`).

---

## 5. GitHub Actions: does changing a workflow `run:` line require any other change?

**Answer: No other workflow change is required.** A `run:` value is self-contained; the edited workflow is picked up from the commit/ref of the triggering event, and, if the step has no explicit `name`, the displayed step name updates automatically from the `run` text. The only conditions are the normal ones: the workflow file must exist on the ref that triggers the run (and, for the events that require it, on the default branch).

Source A — GitHub Docs § About workflows,
URL: https://docs.github.com/en/actions/writing-workflows/about-workflows
(accessed 2026-11-16). Official text:

> The following steps occur to trigger a workflow run:
>
> 1. An event occurs on your repository. The event has an associated commit SHA and Git ref.
> 2. GitHub searches the `.github/workflows` directory in the root of your repository for workflow files that are present in the associated commit SHA or Git ref of the event.
> 3. A workflow run is triggered for any workflows that have `on:` values that match the triggering event. Some events also require the workflow file to be present on the default branch of the repository in order to run.
>
> Each workflow run will use the version of the workflow that is present in the associated commit SHA or Git ref of the event.

Source B — GitHub Docs § Workflow syntax for GitHub Actions → `jobs.<job_id>.steps[*].run`,
URL: https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idstepsrun
(accessed 2026-11-16). Official text:

> Runs command-line programs that do not exceed 21,000 characters using the operating system's shell. If you do not provide a `name`, the step name will default to the text specified in the `run` command.
>
> Each `run` keyword represents a new process and shell in the runner environment. When you provide multi-line commands, each line runs in the same shell.

Source C — GitHub Docs § Workflow syntax → `on.workflow_dispatch`,
URL: https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#onworkflow_dispatch
(accessed 2026-11-16). Official text:

> This trigger only receives events when the workflow file is on the default branch.

**Conclusion:** Editing only the `run:` line is sufficient — there is no version field, registration step, or companion edit documented for a step command change. Consequences to keep in mind: (a) runs triggered from older commits/refs still use the workflow version at *those* refs, per Source A; (b) if the step declares an explicit `name:`, that label will not change when the command changes (cosmetic, not required by the docs); (c) for `workflow_dispatch` and the other default-branch-gated events, the edited file must be on the default branch to be triggerable.

---

## Multilingual official translations (checked, not found at canonical locale paths)

The task requested multilingual official translations "if available." The following official locale-prefixed URLs were checked on 2026-11-16 and returned HTTP 404, so no translated text was used as evidence:

- https://doc.rust-lang.org/book/pt-br/ch11-00-testing.html
- https://doc.rust-lang.org/pt-BR/book/ch11-02-running-tests.html
- https://doc.rust-lang.org/pt-BR/cargo/reference/workspaces.html
- https://rust-lang.github.io/book/pt-br/ch11-02-running-tests.html

The Cargo Book pages consulted expose only a theme selector (Auto / Light / Rust / Coal / Navy / Ayu), not a language selector. The English `doc.rust-lang.org` and `docs.github.com` pages quoted above are therefore the authoritative text for this research note.

---

## Condensed findings (for downstream agents)

| Question | Verified answer | Primary source |
|---|---|---|
| Bare `cargo test`, non-virtual workspace, root package, no `default-members` | Only the root package is selected; all-members happens only for virtual manifests or when `default-members` says so | https://doc.rust-lang.org/cargo/commands/cargo-test.html#package-selection |
| `--workspace` | "Test all members in the workspace" (`--all` is a deprecated alias) | https://doc.rust-lang.org/cargo/commands/cargo-test.html#option-cargo-test---workspace |
| `#[ignore]` tests by default | Compiled but not executed; `-- --ignored` runs only them, `-- --include-ignored` runs all | https://doc.rust-lang.org/reference/attributes/testing.html#the-ignore-attribute ; https://doc.rust-lang.org/rustc/tests/index.html#--ignored |
| `--all-targets` | `--lib --bins --tests --benches --examples`; doc tests are separate (`--doc`), so not included | https://doc.rust-lang.org/cargo/commands/cargo-test.html#option-cargo-test---all-targets |
| Changing a workflow `run:` line | No other change required; each run uses the workflow version at its commit/ref; step name auto-derives from `run` if `name` omitted | https://docs.github.com/en/actions/writing-workflows/about-workflows ; https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idstepsrun |
