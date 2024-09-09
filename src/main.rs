#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
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

    loop {}
}