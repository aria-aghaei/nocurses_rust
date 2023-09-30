extern crate nocurses_rust;

fn main() {
    nocurses_rust::clrscr();
    println!("hit enter");
    nocurses_rust::wait();

    println!("now give me a character");
    let c = nocurses_rust::getch().expect("could not get character");
    println!("your character was: {}", c);
}
