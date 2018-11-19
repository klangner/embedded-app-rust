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

#[allow(non_snake_case)]
#[no_mangle]
pub fn HardFault(_ef: *const u32) -> ! {
    loop {}
}