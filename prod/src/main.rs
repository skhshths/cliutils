mod lib;

use lib::input;
use lib::index;

use lib::show_vec;

use lib::clear;
use lib::clear_win;

fn main() {
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];

    let n: String = show_vec(&x, " - ");
    println!("{n}")
}
