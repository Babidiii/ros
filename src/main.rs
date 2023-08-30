#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Called on panic
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

/// entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
