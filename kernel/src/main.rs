#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}

//use core::fmt::Write;

mod macros;

use bootloader_api::{entry_point, BootInfo, BootloaderConfig, /* config::Mapping */};
mod framebuffer;
use framebuffer::FRAMEBUFFERWRITER;
use core::panic::PanicInfo;
 
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let /* mut */ config = BootloaderConfig::new_default();
    //config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(main, config = &BOOTLOADER_CONFIG);

fn main(bootinfo: &'static mut BootInfo) -> ! {
    FRAMEBUFFERWRITER.lock().init(bootinfo.framebuffer.as_mut().unwrap());
    println!("hello world {}", "!");

    #[cfg(test)]
    test_main();
    
    loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}