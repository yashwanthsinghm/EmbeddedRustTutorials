
//!
//! Example showing the exception ,where systick timer is wrongly configured and exception occurs
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
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprint;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    // configures the system timer to trigger a SysTick exception every second
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // period = 1s
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}


///exception handlerfor systick
#[exception]
fn SysTick() {
    hprint!(".").unwrap();
}