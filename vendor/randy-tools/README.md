# randy-tools

[![codecov badge][]][codecov link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`randy-util` is a set of utility types and functions for randy based on [`twilight-rs`]'s `twilight-utils`.
Do not use it unless you know what you are doing !

## Features

### `builder`

Provides builders for large structs.

### `link`

Provides implementations for parsing and formatting entities' URLs, such as
webhook URLs.

### `permission-calculator`

Allows the use of a calculator to determine the permissions of a member in
a guild or channel.

### `snowflake`

Allows the use of the `Snowflake` trait, which provides methods for the extraction of
structured information from [Discord snowflakes].

[`twilight-rs`]: https://github.com/twilight-rs/twilight
[codecov badge]: https://img.shields.io/codecov/c/gh/randy-rs/randy?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/randy-rs/randy/
[github badge]: https://img.shields.io/badge/github-randy-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/randy-rs/randy
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/randy-rs/randy/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
[Discord snowflakes]: https://discord.com/developers/docs/reference#snowflakes
