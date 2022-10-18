#![allow(dead_code)]

//! # fs-rs native
//! 
//! A sandbox library for trying out F#-to-Rust interop.
//! Just a personal playground :)

/// Internal logic designed for idiomatic Rust consumption.
pub mod implementation;

/// External wrapper designed for C interop.
pub mod interface;
