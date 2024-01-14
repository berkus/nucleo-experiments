#![no_std]
#![no_main]

use {
    cortex_m_rt::entry,
    debounced_pin::{prelude::*, ActiveLow},
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
    let mut button = DebouncedInputPin::new(button, ActiveLow);

    let mut delay = 10_0000_u32;

    led.set_low();

    loop {
        delay = loop_delay(delay, &mut button);
        led.toggle();
    }
}

fn loop_delay<const P: char, const N: u8>(
    mut delay: u32,
    button: &mut DebouncedInputPin<Pin<P, N>, ActiveLow>,
) -> u32 {
    defmt::println!("Waiting for {}", delay);
    for _ in 1..delay {
        match button.update().expect("whatever") {
            DebounceState::Active => {
                delay -= 2_5000_u32;
                if delay < 2_5000_u32 {
                    delay = 10_0000_u32;
                }
                defmt::println!("delay: {}", delay);
                return delay;
            }
            _ => (),
        }
    }
    delay
}
