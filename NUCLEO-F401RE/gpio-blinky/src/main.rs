#![no_std]
#![no_main]

use {
    cortex_m_rt::entry,
    defmt_rtt as _, panic_probe as _,
    stm32f4xx_hal::{
        gpio::Pin,
        hal::digital::v2::InputPin,
        pac::{self},
        prelude::*,
    },
    unflappable::{debouncer_uninit, default::ActiveLow, Debounced, Debouncer},
};

static DEBOUNCER: Debouncer<Pin<'C', 13>, ActiveLow> = debouncer_uninit!();

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13;

    let debounced_button =
        unsafe { DEBOUNCER.init(button) }.expect("failed to initialize debouncer");

    let mut delay = 10_0000_u32;

    led.set_low();

    loop {
        delay = loop_delay(delay, &debounced_button);
        led.toggle();
    }
}

fn loop_delay(mut delay: u32, debounced_button: &Debounced<'_, ActiveLow>) -> u32 {
    defmt::println!("Waiting for {}", delay);
    for _ in 1..delay {
        unsafe {
            DEBOUNCER.poll().expect("poll failed");
        }
        if debounced_button.is_low().expect("is high failed") {
            delay -= 2_5000_u32;
            if delay < 2_5000_u32 {
                delay = 10_0000_u32;
            }
            defmt::println!("delay: {}", delay);
            return delay;
        }
    }
    delay
}
