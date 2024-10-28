#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(mini_os::test_runner)]
#![reexport_test_harness_main = "test_main"]


use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};
use mini_os::{memory, println};
use x86_64::{structures::paging::Translate, VirtAddr};

entry_point!(kernel_main); // add type check for bootinfo argument since _start is called outside from bootloader 

#[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Operation System starting...");

    mini_os::init();

    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // using whole physical memory mapping
    let addresses = [
        0xb8000, //the identity mapped vga buffer page
        0x201008, // some code page
        0x0100_0020_1a10, // some stack page
        boot_info.physical_memory_offset, //virtual address mapped to physical address 0
    ];
    let mapper = unsafe {
        memory::init(phys_mem_offset) //init a mapper
    };

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} ---> {:?}", virt, phys);
    }

    /* 
    // use x86_64::registers::control::Cr3;

    use mini_os::memory::active_lvl_4_table;
    use x86_64::VirtAddr;

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {
        active_lvl_4_table(phys_mem_offset)
    };

    for (i, entry) in l4_table.iter().enumerate(){
        if !entry.is_unused() {//only print unused entry
            println!("Level 4 Entry {}: {:?}", i, entry);

            // get the physical address from the entry an dconvert it, 
            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable =  unsafe {
                &*ptr    //level 3 table
            };

            // keep doing this, can get level 2/1 entry as well
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused(){
                    println!("Level 3 Entry {}: {:?}", i , entry);
                }
            }


        }
    }

    use mini_os::memory::translate_addr;


    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset)};

        println!("{:?} ---> {:?}", virt, phys);
    }
    */

    /* find level 4 page table address
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());
    */
    /*
        PAGE FAULT
        assign a value read-only address
        // let ptr = 0xdeadbeaf as *mut u8;
        let ptr = 0x20422c as *mut u8; // a code page is read-only
        
        unsafe {
            let x = *ptr; //read successfully
            println!("read got: {}",x);
        }
        println!("Read ok");
        unsafe { *ptr = 99; }
    */
        
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

    mini_os::hlt_loop();

    /* 
    loop {
        // call crate print to create a deadlock
        use mini_os::print; //call _print where has a WRITER lock inside

        for _ in 0..100000{}  //add loop to slow down then show print lock and time interrupt work together

        print!("-"); //since lock is occupied here, time handler can't get the lock anymore
    }
    */
}

// panic funtion
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    println!("{}", info);
    mini_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    mini_os::test_panic_handler(info);
}




