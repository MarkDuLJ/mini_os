#![no_std]
#![no_main]

// use core::panic::PanicInfo;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> !{
    loop {
        
    }
}