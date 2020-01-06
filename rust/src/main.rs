#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let mut a = 0u32;

    loop {
        a = a+1;
    }
}
