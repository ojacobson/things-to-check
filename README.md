# Things To Check

[![Build Status](https://travis-ci.org/ojacobson/things-to-check.svg?branch=main)](https://travis-ci.org/ojacobson/things-to-check)

A friend of mine used to run an IRC bot that could provide "helpful"
troubleshooting suggestions, based on places the folks in that chat had stubbed
their toes in the past.

I thought this was such a good idea, I turned it into a web bot.

## You Will Need

Want to work on this code, or run it yourself? Install the following:

* [An installed copy of the Rust toolchain](https://rustup.rs).

Building, testing, &c follow Rust norms: use
[Cargo](https://doc.rust-lang.org/cargo/guide/working-on-an-existing-project.html)
to build, test, or run the code locally.

## Configuration

By default, this app will listen on `http://localhost:3000/`. You can change the
port number by exporting a `PORT` environment variable in the process where this
program runs.

## Vocabulary

The list of suggestions is given by the `src/things-to-check.yml` file, which
contains a YAML list of strings. Each string is a Markdown snippet to render in
the page as a suggestion.

Stable links provide the user with an index into this list. When you insert new
items, insert them at the end.

## Git hooks

This project includes a pre-commit and a pre-merge-commit hook to run tests.
Using this hook is optional, but it'll catch a lot of minor breakage before
Travis does.

**Security note**: Enabling in-repository hooks means that anyone who can commit
code to this repository can run that code on your computer. Only do this if
you're willing to take that chance.

To set this up:

* Install additional Rust components:

    ```bash
    rustup component add clippy rustfmt
    ```

* Install `cargo-udeps`:

    ```bash
    cargo install cargo-udeps
    ```

* Configure Git to use these hooks:

    ```bash
    git config core.hooksPath .git-hooks
    ```

This only needs to be done once, and applies only to this project. To undo this,
unset `core.hooksPath`:

```bash
git config --unset core.hooksPath
```

You can also temporarily suppress the hook with `git commit --no-verify`, if you
have broken code you want to check in, or if the internet is unavailable for
Cargo to download dependencies.
