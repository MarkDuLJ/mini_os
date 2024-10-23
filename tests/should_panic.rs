#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop{}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
        serial_println!("[Test not panic]");
        exit_qemu(QemuExitCode::Failed);
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn fail_case() {
    serial_println!("fail_case::failure case...\t");
    assert_eq!(1,3);
}

use core::panic::PanicInfo;
use mini_os::{QemuExitCode, exit_qemu, serial_println};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);

    loop {}
}