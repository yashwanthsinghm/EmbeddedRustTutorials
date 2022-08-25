//! Interrupt in stm32
#![deny(missing_docs)]
#![deny(arithmetic_overflow)]
#![deny(warnings)]
//#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![allow(warnings)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_main]
#![no_std]

use panic_halt as _;
use stm32f4xx_hal as hal;

use crate::hal::{
gpio::{self,Output,PushPull},
pac::{interrupt,Interrupt,Peripherals,TIM2},
prelude::*,
timer::{CounterUs,Event},
};

use core::cell::RefCell;
use cortex_m_rt::entry;
use cortex_m::interrupt::Mutex;

// For the onboard nucleo LED, use PA5 or PB13 depending your model
type LedPin = gpio::PA5<Output<PushPull>>;

// Make LED pin globally available
static G_LED: Mutex<RefCell<Option<LedPin>>> = Mutex::new(RefCell::new(None));

// Make timer interrupt registers globally available
static G_TIM: Mutex<RefCell<Option<CounterUs<TIM2>>>> = Mutex::new(RefCell::new(None));

// Define an interupt handler, i.e. function to call when interrupt occurs.
// This specific interrupt will "trip" when the timer TIM2 times out
///Interrupt handler
#[interrupt]
fn TIM2() {
    static mut LED: Option<LedPin> = None;
    static mut TIM: Option<CounterUs<TIM2>> = None;

    let led = LED.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            G_LED.borrow(cs).replace(None).unwrap()
        })
    });

    let tim = TIM.get_or_insert_with(|| {
        cortex_m::interrupt::free(|cs| {
            // Move LED pin here, leaving a None in its place
            G_TIM.borrow(cs).replace(None).unwrap()
        })
    });

    led.toggle();
    let _ = tim.wait();
}

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).pclk1(8.MHz()).freeze();

    // Configure PA5 pin to blink LED
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();
    led.set_high(); // Turn off

    // Move the pin into our global storage
    cortex_m::interrupt::free(|cs| *G_LED.borrow(cs).borrow_mut() = Some(led));

    // Set up a timer expiring after 1s
    let mut timer = dp.TIM2.counter(&clocks);
    timer.start(1.secs()).unwrap();

    // Generate an interrupt when the timer expires
    timer.listen(Event::Update);

    // Move the timer into our global storage
    cortex_m::interrupt::free(|cs| *G_TIM.borrow(cs).borrow_mut() = Some(timer));

    //enable TIM2 interrupt
    unsafe {
        cortex_m::peripheral::NVIC::unmask(Interrupt::TIM2);
    }

    #[allow(clippy::empty_loop)]
    loop {
        // Uncomment if you want to make controller sleep
        // cortex_m::asm::wfi();
    }
}
