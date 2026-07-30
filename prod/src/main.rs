#![allow(dead_code, unused_imports, unused_variables, special_module_name)]

mod testlib;

use std::collections::HashMap;

use testlib::vec_to_string;
use testlib::hash_to_vec;
use testlib::clear_win;

fn main() {
    let x: Vec<i32> = vec![1, 2, 3, 4, 5];

    let mut b: HashMap<&str, i32> = HashMap::new();
    b.insert("oliver", 100);
    b.insert("blaise", 5000);

    let n: Vec<String> = hash_to_vec(&b, " - ");

    for x in n {
        println!("{x}");
    }
}
