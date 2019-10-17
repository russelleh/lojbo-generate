mod root_word_value;

use std::fs;
use root_word_value::RootWordValue;

fn main() {
  let data = fs::read_to_string("data/gismu.txt").expect("err");

  let lines:  Vec<&str>           = data.split("\n").collect();
  let values: Vec<RootWordValue>  = lines.iter().filter_map(|line| {
    let value = line.chars().skip(1).take(5).collect();
    RootWordValue::new(value).ok()
  }).collect();

  values.iter().for_each(|value| {
    println!("{}", value)
  })
}
