#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]


mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop{}
}

// panic handler in test mode
#[panic_handler]
#[cfg(test)]
fn panic(info:&PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Erro info: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {
        
    }
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
    serial_println!("Running {} tests",tests.len());
    println!("Running {} tests...", tests.len());
    for test in tests {
        test();
    }

    println!("quiting system");
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn demo_assertion() {
    serial_print!("demo assertion using serial print marco");
    print!("an assertion demo...");
    assert_eq!(1,2);
    println!("[OK]");
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
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}