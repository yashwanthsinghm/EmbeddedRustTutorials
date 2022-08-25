//! Changing the panicking behavior
//!
//! The easiest way to change the panicking behavior is to use a different [panic handler crate][0].
//!
//! [0]: https://crates.io/keywords/panic-impl
#![deny(missing_docs)]
#![deny(arithmetic_overflow)]
#![deny(warnings)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![allow(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_main]
#![no_std]

// Pick one of these panic handlers:

// `panic!` halts execution; the panic message is ignored
use panic_halt as _;

// Reports panic messages to the host stderr using semihosting
// NOTE to use this you need to uncomment the `panic-semihosting` dependency in Cargo.toml
// use panic_semihosting as _;

// Logs panic messages using the ITM (Instrumentation Trace Macrocell)
// NOTE to use this you need to uncomment the `panic-itm` dependency in Cargo.toml
// use panic_itm as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    panic!("Oops")
}