#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use rt::entry;

entry!(main);


fn main() -> ! {
    // This will case HardFault exception
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub extern "C" fn HardFault() -> ! {
    loop {}
}