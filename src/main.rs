#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]


mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}


#[no_mangle]
pub extern "C" fn _start() -> !{
/*
    let vga_buf = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xb;
        }
    }
 */

// vga_buffer::WRITER.lock().write_string("Hello here");

println!("Hello {}", "...");
// panic!("here is a panic coming...");

#[cfg(test)]
test_main();

    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }
}

#[test_case]
fn demo_assertion() {
    print!("an assertion demo...");
    assert_eq!(1,1);
    println!("[OK]");
}