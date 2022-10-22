#![allow(clippy::large_enum_variant)]
#![no_std]

pub mod draw;
pub mod game;
pub mod maze;
#[cfg(target_os = "macos")]
pub mod time;
