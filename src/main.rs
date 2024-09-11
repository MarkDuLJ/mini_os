#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mini_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use mini_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to MINI_OS");

    // init for interrupt
    mini_os::init();

    // invoke a breakpoint interruption
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    loop {
        
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mini_os::test_panic_handler(info);
}

