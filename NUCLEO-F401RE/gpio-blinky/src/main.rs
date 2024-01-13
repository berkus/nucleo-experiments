#![no_std]
#![no_main]

use {
    cortex_m_rt::entry,
    panic_halt as _,
    stm32f4xx_hal::{
        gpio::Pin,
        pac::{self},
        prelude::*,
    },
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    let gpioc = dp.GPIOC.split();
    let button = gpioc.pc13;

    let mut delay = 10_0000_u32;

    led.set_low();

    loop {
        delay = loop_delay(delay, &button);
        led.toggle();
    }
}

fn loop_delay<const P: char, const N: u8>(mut delay: u32, button: &Pin<P, N>) -> u32 {
    for _ in 1..delay {
        if button.is_low() {
            delay -= 2_5000_u32;
            if delay < 2_5000_u32 {
                delay = 10_0000_u32;
            }
            return delay;
        }
    }
    delay
}
