#![no_std]
#![no_main]

use {
    cortex_m_rt::entry,
    debouncr::{debounce_stateful_16, DebouncerStateful, Repeat16},
    defmt_rtt as _, panic_probe as _,
    stm32f4xx_hal::{
        gpio::Pin,
        pac::{self},
        prelude::*,
    },
};

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

    let mut button_state = debounce_stateful_16(false);

    let mut delay = 10_0000_u32;

    led.set_low();

    loop {
        delay = loop_delay(delay, &button, &mut button_state);
        led.toggle();
    }
}

fn loop_delay<const P: char, const N: u8>(
    mut delay: u32,
    button: &Pin<P, N>,
    button_state: &mut DebouncerStateful<u16, Repeat16>,
) -> u32 {
    defmt::println!("Waiting for {}", delay);
    for _ in 1..delay {
        let is_pressed = button.is_low();
        button_state.update(is_pressed);
        if button_state.is_high() {
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
