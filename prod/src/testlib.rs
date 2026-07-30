#![allow(dead_code)]

use std::io;
use std::io::Write;

use std::collections::HashMap;

use std::fmt::Display;

use std::cmp::PartialEq;

use std::process::Command;

pub fn input(prompt: &str) -> &str {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let x: &str = "";
  io::stdin().read_line(&mut x.to_string()).unwrap();
  x.trim()
}

pub fn clear() -> () {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

pub fn clear_win() -> () {
  Command::new("cmd")
    .args(["/C", "cls"])
    .status()
    .unwrap();  
}

pub fn index<T: PartialEq>(val: T, names: &[T]) -> usize {
  names.iter().position(|x| x == &val).unwrap()
}

pub fn vec_to_string<T: Display>(v: &[T], inter: &str) -> String {
  let mut out: String = String::new();
  for item in v {
    out += &item.to_string();
    out += &inter;
  }
  out.trim_end_matches(&inter.to_string()).to_string()
}

pub fn hash_to_vec<T: Display, V: Display>(h: &HashMap<T, V>, inter: &str) -> Vec<String> {
  let mut out: Vec<String> = Vec::new();
  for item in h {
    out.push(format!("{}{}{}", item.0, inter, item.1));
  }
  out
}

pub fn clean_split<'a>(original: &'a str, split_at: &'a str) -> Vec<&'a str> {
  original.split(split_at).collect::<Vec<&str>>()
} 

pub fn string_to_vec(original: &str) -> Vec<&str> {
  original.split("").filter(|x| x != &"").collect()
}

pub fn strip_all<'a>(original: &'a str, target: &'a str) -> &'a str {
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