#![allow(dead_code)]

//! # fs-rs native
//! 
//! A sandbox library for trying out F#-to-Rust interop.
//! Just a personal playground :)

/// Custom allocator to debug FFI heap corruption.
pub mod allocator;

/// Internal logic designed for idiomatic Rust consumption.
pub mod implementation;

/// External wrapper designed for C interop.
pub mod interface;
