#![feature(panic_handler)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootloader;

use core::instrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe{ instrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {
    loop{}
}
