#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]


pub mod vga_buf;
pub mod serial;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;

pub fn init(){
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize()};//init pic

    x86_64::instructions::interrupts::enable();
}

// trait for print message for every test
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where T: Fn(),  
{
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> !{
    init(); // for cargo test --lib, init IDT before running test
    test_main();
    loop {
        
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[Failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)] // panic handler in test mode
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    test_panic_handler(info);
 

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

