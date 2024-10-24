#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use lazy_static::lazy_static;

use mini_os::{exit_qemu, serial_print, serial_println, QemuExitCode};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64
) ->!
{
    serial_println!("[OK]");
    exit_qemu(QemuExitCode::Success);
    loop {
        
    }
}

lazy_static!{
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(mini_os::gdt::DOUBLE_FAULT_IST_INDEX);//w/o this set, test will fail
        }
        idt
    };
}

pub fn init_test_idt(){
    TEST_IDT.load();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
  serial_print!("stack_overflow::stack_overfow...\t");
  mini_os::gdt::init();// init a new GDT for test
  init_test_idt();

// triger a stack over flow
    stack_overflow();

  panic!("Execution continued after stack overflow.")
}

#[allow(unconditional_recursion)]
fn stack_overflow(){
    stack_overflow();// for each recursion, the return address is pushed
    volatile::Volatile::new(0).read();// avoid recursion optimization
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    mini_os::test_panic_handler(info);
}