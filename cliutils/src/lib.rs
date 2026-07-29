use std::io;
use std::io::Write;

use std::collections::HashMap;

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
pub fn vec_to_string<T: Display>(v: &[T], inter: &str) -> String {
  let mut out: String = String::new();
  for item in v {
    out += &item.to_string();
    out += &inter;
  }
  out.trim_end_matches(&inter.to_string()).to_string()
}

#[allow(dead_code)]
pub fn hash_to_string<T: Display, V: Display>(h: &HashMap<T, V>, inter: &str) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();
  for item in h {
    let mut new: String = String::new();
    new += &item.0.to_string();
    new += inter;
    new += &item.1.to_string();
    out.push(new);
  }
  out
}

#[allow(dead_code)]
pub fn clean_split<'a>(original: &'a str, split_at: &'a str) -> Vec<&'a str> {
  original.split(split_at).collect::<Vec<&str>>()
} 

#[allow(dead_code)]
pub fn string_to_vec(original: &str) -> Vec<&str> {
  original.split("").filter(|x| x != &"").collect()
}

#[allow(dead_code)]
pub fn strip_all<'a>(original: &'a str, target: &'a str) -> &'a str {
  let c: Vec<&str> = string_to_vec(original);
  let mut out: &str = original;
  let mut done_something: bool = false;
  if original.starts_with(target) {
    out = out.strip_prefix(target).unwrap();
    done_something = true;
  }
  if original.ends_with(target) {
    out = out.strip_suffix(target).unwrap();
    done_something = true;
  }
  if !done_something {
    out = original;
  }
  out
}