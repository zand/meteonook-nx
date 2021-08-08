#![no_std]

use core::panic;

#[panic_handler]
fn panic_handler(_info: &panic::PanicInfo) -> ! {
    loop {}
}

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

