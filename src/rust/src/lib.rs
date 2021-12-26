//! TODO: Write crate docs

// #![doc(html_root_url = "https://docs.rs/{{CRATE}}/0.0.0")]
// Deny a few warnings in doctests, since rustdoc `allow`s many warnings by default

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod rustrlr;
mod readme;
mod rustr;
