#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

#[no_mangle]
#[link_section = ".htif"]
pub static mut tohost: u64 = 0;

#[no_mangle]
#[link_section = ".htif"]
pub static fromhost: u64 = 0;

#[entry]
fn main() -> ! {
    let mut a = 0u32;

    let _b = fromhost;

    loop {
        // __set_tohost(dev=1, cmd=1, char);
        if cfg!(target_arch="riscv64") {
            unsafe {
                tohost = 1u64 << 56 | 1u64 << 48 | 0x41u64;
            }
        }
    }
}
