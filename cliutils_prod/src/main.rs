mod lib;

use lib::input;

fn main() {
    let x: String = input("name: ");
    println!("hi {x}");
}
