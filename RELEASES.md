HarTex 1.22.0 (10 November 2021)
================================

Key Changes
-----------
- code improvements (rustfmt & clippy)
- create `hartex_env` crate
- panic handler added

HarTex 1.21.0 (8 October 2021)
================================

Key Changes
----------------

- [removed `hartex_logging` crate in favour of `tracing`][hartex pr#33]
- [removed the pre-command check API][hartex commit#7d786ac]
- [use the stabilized Rust 2021 Edition][hartex commit#a5eb426]
- [add `userinfo` command][hartex commit#1a230d4]
- [add `guildinfo` command][hartex commit#1b20acf]

[hartex pr#33]: https://github.com/HarTexBot/HarTex-rust-discord-bot/pull/33
[hartex commit#7d786ac]: https://github.com/HarTexBot/HarTex-rust-discord-bot/commit/7d786ac50f7051999df84d785da994e421388562
[hartex commit#a5eb426]: https://github.com/HarTexBot/HarTex-rust-discord-bot/commit/a5eb42607665685a2ef5d1c9a146999f89c183c5
[hartex commit#1a230d4]: https://github.com/HarTexBot/HarTex-rust-discord-bot/commit/1a2e0d447263e5b5a40cd6168b02fe6653b87f3d
[hartex commit#1b20acf]: https://github.com/HarTexBot/HarTex-rust-discord-bot/commit/1b20acf939f0097cc83057c4b7ddd0ba79dad013