#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let period = 50_u16;
    let mut tick = 0;
    leds[0].on();

    loop {
        if tick % 2 == 0 {
            let led_to_turn_on = (tick / 2 + 1) % 8;
            leds[led_to_turn_on].on();
        }

        if tick % 2 == 1 {
            let led_to_turn_off = (tick / 2) % 8;
            leds[led_to_turn_off].off();
        }

        delay.delay_ms(period);
        tick = (tick + 1) % 16;
    }
}
