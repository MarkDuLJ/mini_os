#![no_std]
#![no_main]

use core::fmt::Write;

mod vga_buf;

// use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    panic!("panic happens here...");
    loop {
        
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> !{
    println!("{}", info);
    loop {
        
    }
}