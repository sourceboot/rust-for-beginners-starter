# rust-for-beginners-starter

The starter workspace for **Rust for Beginners: Build a Word Game** — a
[SourceBoot](https://sourceboot.com) course that assumes **no programming
experience at all**: a terminal, the patience to read what a compiler says, and
nothing else. You build one real thing, lab by lab: a terminal word-guessing game
— a secret word, per-letter feedback, six tries — that you can hand to a friend
when it is done.

Already write code in some language? This course will feel slow to you — start at
[Rust for Systems](https://sourceboot.com/courses/rust-for-systems) instead, which
assumes a working developer and zero Rust.

This repo is exactly what a learner's workspace starts as: a plain cargo workspace
holding one library crate, `lantern`, with a module per lab — most of them empty
files waiting for their lab, one of them (the word list) shipped complete. The
lessons and the grading are deliberately **not** in here — they live on
[sourceboot.com](https://sourceboot.com), and `sboot test` checks your work. A repo
created from this template stays your code and nothing else, which is what makes it
worth showing people.

> Renamed 2026-09-01 (was `rust-start-starter`, when the course id was `rust-start`). GitHub
> redirects renamed repos, so a template link you already have keeps working. The
> crate name `lantern` is unchanged.

## Use it

Two ways in; both give you the same tree.

**With GitHub** — your game starts life as a private repo you own:

```sh
gh repo create my-word-game --private --template sourceboot/rust-for-beginners-starter --clone
cd my-word-game
```

Then install `sboot` and work from inside the clone:

```sh
curl -fsSL https://sourceboot.com/install.sh | sh
sboot login                   # connects this machine, in your browser
sboot test 00-welcome         # checks your work on lab 00
```

`sboot` recognises the repo by its `sboot.toml`. Running
`sboot start rust-for-beginners` inside the clone is safe: it puts back anything
that is missing and never touches a file you have edited. With the template you
already have the tree, so you don't need it.

**Without GitHub:**

```sh
sboot start rust-for-beginners
```

materialises this same tree into `./sourceboot-rust-for-beginners/` — named after
the course you typed — makes it a git repository and commits it, asking
once for the name and email git stamps on your commits. No `gh` and no template
involved; `--dir <name>` picks a different folder. When you want it on GitHub,
`sboot repo` creates the private repo and pushes it.

## What's in the tree

```
game/                   the cargo workspace you own
  lantern/              the game you write, one lab at a time
    src/                a module per lab — banner, guess, rounds, words,
                        score, tracker, game. banner.rs ships its three
                        constants, the rest are empty until their lab;
                        lib.rs declares each one as the labs tell you to
    src/wordlist.rs     the course word list — ships complete, never edited,
                        proven sound by its own test
    src/main.rs         the playable shell — grows a few lines every lab
.cargo/                 the course's build settings — shipped, never edited
rust-toolchain.toml     the current stable Rust, plus its code checker, clippy
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
your first function and your first test, and leads you to two compile errors on
purpose, so you learn to read one with the lesson beside you. When something
turns red mid-lab, that is the course working, not this template failing.

## The course

https://sourceboot.com — lessons, labs and grading. This template is just the
starting tree.

## License

MIT — see [LICENSE](LICENSE). The scaffold is yours to build on and publish.
The course prose, tests and grader are not in this repo and are not covered by
it.
