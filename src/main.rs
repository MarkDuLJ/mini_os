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

    // cause a page fault
    let ptr = 0xdeadbeaf as *mut u8;
    unsafe { *ptr = 43;}

    
/* 
    // invoke a breakpoint interruption
    x86_64::instructions::interrupts::int3();
*/

/* 
    // trigger a page fault(one kind of double fault), finally cause a fatal triple fault.
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }
*/
    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    mini_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
   mini_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mini_os::test_panic_handler(info);
}

