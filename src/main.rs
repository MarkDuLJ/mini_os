#![no_std]
#![no_main]

// use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buf = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe {
            *vga_buf.offset(i as isize * 2) = byte;
            *vga_buf.offset(i as isize * 2 + 1) = 0xc;
        }
    }

    loop {
        
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !{
    loop {
        
    }
}