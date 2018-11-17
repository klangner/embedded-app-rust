#![no_std]
#![no_main]

extern crate rt;

use rt::entry;

entry!(main);

pub fn main() -> ! {
    let _x = 42;

    loop {}
}
