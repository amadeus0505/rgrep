use std::io::Write;
use winapi::um::wincon::SetConsoleTextAttribute;
use winapi::um::processenv::GetStdHandle;

pub enum Color{
    Black,
    DarkBlue,
    Green,
    LightBlue,
    Red,
    Purple,
    Yellow,
    White,
    Grey,
    Blue,
}

impl Color {
    fn value(&self) -> u16 {
        match *self {
            Color::Black => 0,
            Color::DarkBlue => 1,
            Color::Green => 2,
            Color::LightBlue => 3,
            Color::Red => 4,
            Color::Purple => 5,
            Color::Yellow => 6,
            Color::White => 7,
            Color::Grey => 8,
            Color::Blue => 9,
        }
    }
}

pub fn main() {
    cprint("Dies ist normaler Text".to_string(), Color::White);
    cprint("Dies ist roter Text".to_string(), Color::Red);
    cprint("Dies ist wieder normaler Text".to_string(), Color::White);
}

pub fn cprintln(mut string: String, color: Color) {
    string.push_str("\n");
    cprint(string, color);
}

pub fn cprint(string: String, color: Color) {
    let handle = unsafe { GetStdHandle(-11i32 as u32) };
    let mut stdout = std::io::stdout();
    let attributes = color.value();

    // flush before setting Console-Color (see below for explanation)
    stdout.flush().expect("Flush stdout failed");
    // setting console-color
    unsafe { SetConsoleTextAttribute(handle, attributes) };

    /*
        Rust writes to stdout line per line, so multiple colors in a single line won't work
        with flush() we can force rust to write to the cmd -> multiple colors per line are now possible
     */
    stdout.write_all(string.as_bytes()).unwrap();
    stdout.flush().expect("Flush stdout failed");
}
