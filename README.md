# rust-start-starter

The starter workspace for **Rust from Zero: Build a Word Game** — a
[SourceBoot](https://sourceboot.com) course that assumes **no programming
experience at all**: a terminal, the patience to read what a compiler says, and
nothing else. You build one real thing, lab by lab: a terminal word-guessing game
— a secret word, per-letter feedback, six tries — that you can hand to a friend
when it is done.

Already write code in some language? This course will feel slow to you — start at
[rust-core](https://sourceboot.com) instead, which assumes a working developer and
zero Rust.

This repo is exactly what a learner's workspace starts as: a plain cargo workspace
holding one library crate, `lantern`, with a module per lab — most of them empty
files waiting for their lab, one of them (the word list) shipped complete. The
lessons, the per-lab tests and the grader are deliberately **not** in here — they
live on [sourceboot.com](https://sourceboot.com) and arrive through the `sboot`
CLI, into a separate cache directory. A repo created from this template stays your
code and nothing else, which is what makes it worth showing people.

> Naming note: `rust-start-starter`, the course id `rust-start`, and the crate
> name `lantern` are working names. They may be renamed before launch; GitHub
> redirects renamed repos.

## Use it

Two ways in; both give you the same tree.

**With GitHub** — your game starts life as a private repo you own:

```sh
gh repo create my-word-game --private --template sourceboot/rust-start-starter --clone
cd my-word-game
```

Then install `sboot` and work from inside the clone:

```sh
curl -fsSL https://sourceboot.com/install.sh | sh
export SBOOT_TOKEN=...        # from https://sourceboot.com/account
sboot test 00-welcome         # fetches the lab's tests + grader, runs them, grades
```

`sboot` recognises the repo by its `sboot.toml` and downloads each lab's tests on
first use (`sboot where` prints where they live — outside this repo). Note: don't
run `sboot start` inside the clone — that command creates a fresh `./rust-start/`
directory and refuses to write into a non-empty one. With the template you already
have the tree, so you don't need it.

**Without GitHub:**

```sh
sboot start rust-start
```

materialises this same tree into `./rust-start/`, no `gh` and no template involved
— make it a git repo whenever you like.

## What's in the tree

```
game/                   the cargo workspace you own
  lantern/              the game you write, one lab at a time
    src/                a module per lab — banner, guess, rounds, words,
                        score, tracker, game. All EMPTY until their lab;
                        lib.rs declares each one as the labs tell you to
    src/wordlist.rs     the course word list — ships complete, never edited,
                        proven sound by its own test
    src/main.rs         the playable shell — grows a few lines every lab
rust-toolchain.toml     pinned stable Rust — that is the whole toolchain
sboot.toml              tells the sboot CLI which course this repo is for
```

Stable Rust, on your own machine, with **no external crates, ever** — not even a
random-number crate: the game picks its word by puzzle number, which is what lets
two players race the same puzzle (`cargo run -- 42`) and lets every test know the
answer. Everything runs straight on your machine.

## A fresh clone already passes a test

Nothing in this template is broken on purpose. A fresh clone builds, and your very
first test run is already green — the shipped word list proving its own rules:

```sh
cd game && cargo test
```

```
running 1 test
test wordlist::tests::the_list_is_sound ... ok
```

That one green test is deliberate: it is the course's first proof that the
build-test loop works on your machine, before you have written any Rust. `cargo
run` prints a placeholder line until lab 01 puts your game's name on the screen.

The first red you meet will be one the course walks you into: lab 01 has you write
your first function and your first test, and lab 02 deliberately leads you to a
compile error so you learn to read one with the lesson beside you. When something
turns red mid-lab, that is the course working, not this template failing.

## The course

https://sourceboot.com — lessons, labs and grading. This template is just the
starting tree.

## License

MIT — see [LICENSE](LICENSE). The scaffold is yours to build on and publish.
The course prose, tests and grader are not in this repo and are not covered by
it.
