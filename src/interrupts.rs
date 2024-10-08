use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;

use pic8259::ChainedPics;
use spin;

use crate::{hlt_loop, print};
use crate::println;
use crate::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe {
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
    }
);

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);//set stack index instead

        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);        
        idt
    };
  
}

pub fn init_idt(){
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame
)
{
    println!("EXCEPTION: BreakPoint\n{:#?}",stack_frame);
}

#[test_case]
fn test_breakpoint_excption(){
    // invoke a breakpoint exception for test
    x86_64::instructions::interrupts::int3();
}

// double fault handler to avoid triple fault
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64
) -> !
{
    panic!("EXCEPTION: Double Fault\n{:#?}", stack_frame);
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,   //index is 1 here, map to interrupt 33(32+1)
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler( _stack_frame: InterruptStackFrame){
    print!(".");

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame){
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    // HandleControl for ctrl + [a-z]
    lazy_static!{
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = 
            Mutex::new(Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore));
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);

    let scancode: u8 = unsafe {
        port.read()
    };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character), //map ctrl + [a-z]
                DecodedKey::RawKey(key) => print!("{:?}",key), //
            }
        }
    }


 /*    
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe {
        port.read()
    };

    // print!("keyboard {}", scancode);

    let key = match scancode {
        0x02 => Some('1'),
        0x03 => Some('2'),
        0x04 => Some('3'),
        0x05 => Some('4'),
        0x06 => Some('5'),
        0x07 => Some('6'),
        0x08 => Some('7'),
        0x09 => Some('8'),
        0x0a => Some('9'),
        0x0b => Some('0'),
        _ => None,
    };

    if let Some(key) = key {
        print!("{}", key);
    }
*/
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
){
    use x86_64::registers::control::Cr2;

    println!("EXCEPTIOM: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:?}", stack_frame);
    hlt_loop();
}