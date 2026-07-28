use std::io;
use std::io::Write;

use std::fmt::Display;

use std::cmp::PartialEq;

use std::process::Command;

#[allow(dead_code)]
pub fn input(prompt: &str) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x: String = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

#[allow(dead_code)]
pub fn clear() -> () {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

#[allow(dead_code)]
pub fn clear_win() -> () {
  Command::new("cmd")
    .args(["/C", "cls"])
    .status()
    .unwrap();  
}

#[allow(dead_code)]
pub fn index<T: PartialEq>(val: T, names: &[T]) -> usize {
  names.iter().position(|x| x == &val).unwrap()
}

#[allow(dead_code)]
pub fn show_vec<T: Display>(v: &[T], inter: String) -> String {
  let mut out: String = String::new();
  for item in v {
    out += &item.to_string();
    out += &inter;
  }
  out
}