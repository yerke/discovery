#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let mut half_period = 500_u16;
    delay.delay_ms(half_period);
    half_period = 100;

    let mut current_led = 0;
    loop {
        leds[current_led].on();
        delay.delay_ms(half_period);

        leds[current_led].off();
        delay.delay_ms(half_period);

        current_led = (current_led + 1) % 8;
    }
}
