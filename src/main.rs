#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"] //redefine test entry point to test_main

use core::fmt::Write;

mod vga_buf;
mod serial;

// use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    loop {
        
    }
}

#[cfg(not(test))] // panic for regular mode
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !{
    println!("{}", info);
    loop {
        
    }
}

#[cfg(test)] // panic handler in test mode
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !{
    serial_println!("[Failed]\n");
    serial_println!("{}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {
        
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn try_assertion(){
    serial_print!("it's a demo test...");
    assert_eq!(2,2);
    serial_println!("[OK]");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port =  Port::new(0xf4);
        port.write(exit_code as u32);
    }
}