# Contributing

Before contributing to this repository, please first discuss the change you wish
to make via an issue of this repository before making a change.
Please note we have a [code of conduct](CODE_OF_CONDUCT.md), please follow it in
all your interactions with the project.

This project also has an [AI policy](AI_POLICY.md). If you use AI tools in your
contribution, you **must** read and follow it.

- [Contributing](#contributing)
  - [Project mission](#project-mission)
  - [Open an issue](#open-an-issue)
    - [Questions](#questions)
    - [Bug reports](#bug-reports)
    - [Feature requests](#feature-requests)
  - [Preferred contributions](#preferred-contributions)
  - [Development setup](#development-setup)
  - [Pull request process](#pull-request-process)
    - [Software guidelines](#software-guidelines)

---

## Project mission

The project mission is to provide a reusable test harness for Internet Computer
canisters built on top of [PocketIC](https://github.com/dfinity/pocketic), so
that IC projects don't need to reimplement their own integration test
infrastructure.

---

## Open an issue

Open an issue when:

- You have questions or concerns regarding the project.
- You have a bug to report.
- You have a feature or a suggestion to improve pocket-ic-harness to submit.

### Questions

If you have a question, open an issue and label it with `question`. If you need
help with your setup, please also add the `help wanted` label.

### Bug reports

If you want to report a bug you've encountered while using pocket-ic-harness,
open an issue and label it with `bug`. Don't set other labels on your issue, not
even priority.

When you open a bug, try to be as precise as possible in describing your issue.
Sometimes it's very easy for maintainers to understand what you're talking
about, but sometimes we might not know what you mean or we might not have the
technical knowledge you think we have. Always provide:

- The version of `pocket-ic-harness` and of `pocket-ic` you're using.
- The PocketIC server version and your operating system.
- A minimal reproduction: the `Canister` implementation and the test involved,
  or a link to a repository containing them.
- The full error output or panic message.

Please note that we don't provide support for old versions of
pocket-ic-harness. Only the latest release is supported.

Maintainers may add additional labels to your issue:

- **duplicate**: the issue is duplicated; the reference to the related issue
  will be added to your description. Your issue will be closed.
- **priority**: this must be fixed asap.
- **sorcery**: it is not possible to find out what's causing your bug, nor is it
  reproducible on our test environments.
- **wontfix**: your bug has a very high ratio between the difficulty to fix it
  and the probability to encounter it, or it just isn't a bug, but a feature.

### Feature requests

Whenever you have a good idea which could improve the project, it is a good idea
to submit it to the project owner. The first thing you should do though is not
to start writing the code, but to understand how pocket-ic-harness works, what
kind of contribution is appreciated and what kind of contribution won't be
considered. Said so, follow these steps:

- Read the contributing guidelines, entirely.
- Think on whether your idea would fit in the project mission and guidelines or
  not.
- Think about the impact your idea would have on the project, especially on the
  public API (`Canister`, `PocketIcTestEnv`, `PocketIcClient` and the
  `#[pocket_ic_harness::test]` macro).
- Open an issue describing your suggestion with accuracy.
- Wait for the maintainer feedback on your idea.

If you want to implement the feature by yourself and your suggestion gets
approved, start writing the code. Remember that the documentation of the project
is published on [docs.rs](https://docs.rs/pocket-ic-harness). Open a PR related
to your issue. See the [pull request process](#pull-request-process) for more
details.

It is very important to follow these steps, since it will prevent you from
working on a feature that will be rejected, and none of us wants to deal with
this situation.

Always keep in mind that your suggestion may be rejected: the maintainer will
always provide feedback on the reasons that brought to the rejection, just try
not to get mad about that.

---

## Preferred contributions

At the moment, these kinds of contributions are more appreciated and should be
preferred:

- Fixes for [issues reported by the community](https://github.com/veeso/pocket-ic-harness/issues).
- Updates to support new `pocket-ic` releases.
- Improvements to documentation and examples.
- Code optimizations: any optimization to the code is welcome.

For any other kind of contribution, especially for new features, please submit a
new issue first.

---

## Development setup

The repository is a Cargo workspace made of the `pocket-ic-harness` library, the
`pocket-ic-harness-macro` proc-macro crate and a set of integration tests under
`integration-tests/`.

Install the following tools:

- The Rust toolchain pinned in `rust-toolchain.toml`, plus the nightly
  toolchain used by `rustfmt`.
- [just](https://github.com/casey/just), to run the project tasks.
- [dprint](https://dprint.dev), to format code, Markdown, TOML and YAML.
- [cargo-deny](https://github.com/EmbarkStudios/cargo-deny), for the dependency
  checks.

To run the integration tests you also need:

- The `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`.
- [ic-wasm](https://github.com/dfinity/ic-wasm/releases).
- `gzip`, which is usually pre-installed.

Then enable the git hooks and check that everything works:

```bash
just setup_githooks
just check
just integration_test
```

Useful tasks:

- `just fmt`: format the code.
- `just clippy`: run clippy with warnings denied.
- `just test`: run unit and doc tests.
- `just test_all`: run unit tests and integration tests.
- `just doc`: build the documentation, denying warnings.

---

## Pull request process

Let's make it simple and clear:

1. Open an issue first, unless your change is a trivial fix.
2. Open a PR using the provided pull request template, filling in the AI
   disclosure section. PRs which don't use the template will not be reviewed.
3. Write proper documentation for your software, compliant with the **rustdoc**
   standard.
4. Write tests for your code. Cover the harness behavior with unit tests, and use
   the integration tests in `integration-tests/pocket-ic-tests` for anything
   involving a real canister.
5. Run `just fmt` and make sure `just check` passes: it runs the format check,
   clippy, the documentation build, `cargo deny` and the tests.
6. Use [Conventional Commits](https://www.conventionalcommits.org) for your
   commit messages and for the PR title. Mark breaking changes with `!`
   (e.g. `feat!: ...`).
7. Don't edit `CHANGELOG.md`: it is generated from the commit history with
   `git-cliff` at release time.
8. Check that the CI for your commits is green.
9. Describe in the PR what you changed and what you have introduced.
10. Wait for a maintainer to review your PR, and address the requested changes.
11. Request the maintainers to merge your changes.

### Software guidelines

In addition to the process described for the PRs, here is a list of guidelines
to follow when writing the code:

1. **Keep the dependency tree small**: this is a library other projects depend
   on, so every dependency we add is a dependency for all of our users. Please
   don't add dependencies that are not strictly necessary, and keep them
   compliant with `cargo deny`.
2. **Keep the harness generic**: the harness must not know anything about the
   canisters under test. Project specific logic belongs to the user's
   `Canister` implementation, not to this crate.
3. **Document the public API**: every public item must have rustdoc
   documentation.
4. **Test units matter**: whenever you implement something new to this project,
   always implement tests which cover as many cases as possible.
5. **Comments are useful**: what's obvious for you might not be for the others,
   and our capacity to work on a code depends mostly on time and experience,
   not on complexity. When in doubt, add a comment explaining _why_ the code
   does what it does.

---

Thank you for any contribution!
Christian Visintin
