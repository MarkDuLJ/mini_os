use volatile::Volatile;

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
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
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
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode,
}

const BUFFER_HIGH: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HIGH],
}

pub struct Writer {
    column_postion: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s:&str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_postion >= BUFFER_WIDTH {
                    self.new_line()
                }

                let row = BUFFER_HIGH - 1;
                let col = self.column_postion;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write( ScreenChar {
                    ascii_char: byte,
                    color_code,
                });

                self.column_postion += 1;
            }
        }
    }

    fn new_line(&mut self){

    }
}

pub fn print_to_screen() {
    let mut writer = Writer{
        column_postion: 0,
        color_code: ColorCode::new(Color::Cyan, Color::DarkGray),
        buffer: unsafe {
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    writer.write_byte(b'H');
    writer.write_string("ello");
    writer.write_string("W@rld");
}

