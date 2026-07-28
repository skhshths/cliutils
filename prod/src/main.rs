mod lib;

use lib::input;
use lib::index;

use lib::clear;
use lib::clear_win;

fn main() {
    let name: i32 = 3;
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];

    clear_win();

    let three_index: usize = index(name, &x);

    println!("{three_index}");
    println!("{name}");
}
