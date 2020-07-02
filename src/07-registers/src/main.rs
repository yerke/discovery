#![no_main]
#![no_std]

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

#[entry]
fn main() -> ! {
    let mut itm = aux7::init().0;

    unsafe {
        // A magic addresses!
        const GPIOE_BSRR: u32 = 0x4800_1018;
        const GPIOE_ODR: u32 = 0x4800_1014;

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *mut u32)
        );

        // Turn on the "North" LED (red)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 9);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *mut u32)
        );

        // Turn on the "East" LED (green)
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << 11);

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *mut u32)
        );

        // Turn off the "North" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (9 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *mut u32)
        );

        // Turn off the "East" LED
        ptr::write_volatile(GPIOE_BSRR as *mut u32, 1 << (11 + 16));

        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *mut u32)
        );
    }

    loop {}
}
