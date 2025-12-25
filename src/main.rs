#![no_std]
#![no_main]

use nucleo_f4::entry;

entry!(main);
fn main() -> ! {
    let _x = 42;

    loop {}
}