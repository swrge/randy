#![doc = include_str!("../README.md")]
#![warn(
    clippy::missing_const_for_fn,
    clippy::pedantic,
    missing_docs,
    unsafe_code
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unnecessary_wraps
)]

pub mod fmt;
pub mod parse;
pub mod timestamp;

#[doc(no_inline)]
pub use fmt::Mention;

#[doc(no_inline)]
pub use parse::ParseMention;

// [package]
// authors.workspace = true
// description = "Utilities for working with mentions in the Twilight ecosystem."
// edition.workspace = true
// homepage = "https://twilight.rs/chapter_1_crates/section_8_first_party/section_2_mention.html"
// include.workspace = true
// keywords = ["twilight"]
// license.workspace = true
// name = "twilight-mention"
// publish = true
// repository.workspace = true
// rust-version.workspace = true
// version = "0.16.0-rc.1"
//
// [dependencies]
// twilight-model = { default-features = false, path = "../twilight-model", version = "0.16.0-rc.1" }
//
// [dev-dependencies]
// criterion = { default-features = false, version = "0.5" }
// static_assertions = { default-features = false, version = "1" }
//
// [[bench]]
// name = "fmt"
// harness = false
// path = "benches/fmt.rs"
