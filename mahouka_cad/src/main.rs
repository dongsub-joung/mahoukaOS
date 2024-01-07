#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
#[cfg_attr(not(feature = "std"), no_std)]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern  "C" fn _start() -> !{
    loop {
        
    }
}