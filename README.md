# rustrunemetrics

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Nix](https://img.shields.io/badge/NIX-5277C3.svg?style=for-the-badge&logo=NixOS&logoColor=white)

> **DISCLAIMER**: I wrote this a proof of concept to try and show my coworker
> what a Rust port of a Golang package would look like. This crate is **not**
> suitable for production use. See the [License](#license) section below for
> more information.

---

A ~~Golang~~ Rust ~~package~~ crate to interact with the
[RuneMetrics API](https://apps.runescape.com/runemetrics/app/welcome).

## Examples

Clone the repo and navigate to the cloned directory:

```bash
git clone https://github.com/johnDeSilencio/rustrunemetrics.git
cd rustrunemetrics/
```

The following assumes that you have
[Rust installed](https://www.rust-lang.org/tools/install).

### Check if a user has maxed all their skills

```bash
cargo run --example is_max -- --player-name "the-player-you-are-interested-in"
```

For example,

```bash
cargo run --example is_max -- --player-name "its dave"
```

```none
"its dave" has maxed. Way to go its dave!
```

### Check what quests the user has unlocked but not yet completed

```bash
cargo run --example unlocked_quests -- --player-name "the-player-you-are-interested-in"
```

For example,

```bash
cargo run --example unlocked_quests -- --player-name "its dave"
```

```none
"its dave" can complete the following unlocked free quests:

  - Benedict's World Tour (miniquest)
  - Field of Screams
  - Great Egg-spectations
  - Heartstealer
  - It's Snow Bother
  - Violet is Blue Too

"its dave" can complete the following unlocked members quests:

  - Dimension of Disaster
  - Dimension of Disaster: Defender of Varrock
  - Dead and Buried
  - Desperate Creatures
  - Dimension of Disaster: Demon Slayer
  - Dimension of Disaster: Shield of Arrav
  - Housing of Parliament
  - My One and Only Lute
  - Osseous Rex
  - Soul Searching
  - That Old Black Magic
  - The General's Shadow (miniquest)
  - Tortle Combat (miniquest)
  - Twilight of the Gods
  - Wandering Ga'al (miniquest)

Get to work its dave!
```

## Nix Users

For Nix users that have installed [`direnv`](https://github.com/direnv/direnv),
you can navigate to this directory and the correct Rust toolchain necessary to
build this crate will be installed.

## License

This crate was forked from
[`lotkey/gorunemetrics`](https://github.com/lotkey/gorunemetrics) which is
licensed under GPL-3.0. This crate is also licensed under GPL-3.0. Because Rust
projects are compiled from source, this crate is only legally compatible with
other GPL-3.0 (or GPL-3.0 compatabile) crates. You probably do not want to use
this in your crate graph.
