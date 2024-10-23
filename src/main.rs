#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mini_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use mini_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Operation System starting...");

    #[cfg(test)]
    test_main();

    /* 
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    */

    /* 
    vga_buf::WRITER.lock().write_str("Hi there").unwrap();
    write!(vga_buf::WRITER.lock(), "some numbers here: {} {}", 33, 1.0/3.0).unwrap();
    write!(vga_buf::WRITER.lock(), "if you miss a train i'm on, you will know that i am gone. you can hear the whistle blow  hundred miles away.")
        .unwrap();

    println!();
    println!();
    println!("print to screen from marco {}", "made by myself");
    println!();
    println!();
    // panic!("panic happens here...");
    */
    loop {
        
    }
}

// panic funtion
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    loop{}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    mini_os::test_panic_handler(info);
}




