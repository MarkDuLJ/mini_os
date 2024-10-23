use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { 
        col_position: 0, 
        color_code: ColorCode::new(Color::Cyan, Color::Black), 
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
   fn new(foreground: Color, background: Color) -> ColorCode {
    ColorCode((background as u8) << 4 | (foreground as u8))
   } 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] // make sure the field ordering
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH];BUFFER_HEIGHT]
}

// type to write to screen
pub struct Writer {
    col_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// use Writer to modify buffer's characteres
impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT -1;
                let col = self.col_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write( ScreenChar {
                    ascii_char: byte,
                    color_code,
                });

                self.col_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }

        self.clear_row(BUFFER_HEIGHT - 1);
        self.col_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code,
        };

        for col in 0..BUFFER_WIDTH{
            self.buffer.chars[row][col].write(blank);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ascii byte or new line
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //  not supported byvga text buffer, print square
                _ => self.write_byte(0xfe),

            }
        }
    }
}

/* 
pub fn print_sth () {
    let mut writer = Writer{
        col_position: 0,
        color_code: ColorCode::new(Color::Cyan, Color::Black),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        }
    };

    use core::fmt::Write;
    writer.write_byte(b'H');
    writer.write_string("elllo ");
    writer.write_string("W^rld!");
    write!(writer, "the numbers are {} and {}", 33, 1.0/3.0).unwrap();
}
*/

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buf::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

#[test_case]
fn test_println_output(){
    let s = "something is good for you";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_char), c);
    }
}



#[test_case]
fn try_assertion(){
    // serial_print!("it's a demo test...");
    assert_eq!(2,2);
    // loop{}
    // serial_println!("[OK]");
}

#[test_case]
fn test_println(){
    println!("test println marco");
}

#[test_case]
fn test_println_many(){
    for _ in 0..200 {

        println!("test println marco");
    }
}
