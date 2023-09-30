use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub const BLACK : i32 = 0;
pub const RED : i32 = 1;
pub const GREEN : i32 = 2;
pub const YELLOW : i32 = 3;
pub const BLUE : i32 = 4;
pub const MAGENTA : i32 = 5;
pub const CYAN : i32 = 6;
pub const WHITE : i32 = 7;

fn read_char() -> io::Result<char> {
    loop {
        if let Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        }) = event::read()?
        {
            return Ok(c);
        }
    }
}

pub fn wait(){
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn clrscr(){
    print!("\x1b[2J\x1b[?6h");
}

pub fn clrline(){
    print!("\x1b[2K\x1bE");
}

pub fn resetcolors(){
    print!("\x1b[2K\x1b[0m");
}

pub fn gotoxy(x: i32, y: i32){
    print!("{}", format!("\x1b[{};{}H",y,x));
}

pub fn setfontcolor(color: i32){
    print!("{}", format!("\x1b[3{}m", color));
}

pub fn setbgrcolor(color: i32){
    print!("{}", format!("\x1b[4{}m", color));
}

pub fn settitle(title: &str){
    print!("{}", format!("\x1b]0;{}\\x7", title));
}

pub fn getch() -> io::Result<char>{
    enable_raw_mode().expect("could not enable raw mode");
	let c = read_char();
	disable_raw_mode().expect("could not disable raw mode");
	c
}

