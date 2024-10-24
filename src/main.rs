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

    mini_os::init();

    /* 
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    */

    /*   
        try to write to a readonly address,triger a page fault,
        w/o page fault handler, a double fault occurs.
        IDT doesn't have double foualt handler,
        cause triple fault that system reset endless.
        by add double fault handler in IDT, avoid triple fault.
        unsafe {
            *(0xdeadbeef as *mut u8) = 43;
        }
    */

    /*
      kernal stack overflow
      when touch guard page, page fault occurs, cpu looks up page fault handler
      and try to push interrupt stack frame onto stack, but stack pointer still points to the guard page
      it will cause 2nd page fault, which causes a double fault
      with current double fault handler, still need to push the interrupt stack from, which still points to the guard page
      this is 3rd page fault, which causes triple fault and system reboot.
      Current double fault handler cant sovle this. Demo here.
      That's the reason of using switching stack.
      
      fn stack_overflow(){
        stack_overflow();
    }
    stack_overflow();
    */

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

    println!("I'm still here, no crash...");//check if return after any exceptions

    loop {
        // call crate print to create a deadlock
        use mini_os::print; //call _print where has a WRITER lock inside

        for _ in 0..100000{}  //add loop to slow down then show print lock and time interrupt work together

        print!("-"); //since lock is occupied here, time handler can't get the lock anymore
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




