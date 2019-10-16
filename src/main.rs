mod root_word;

use std::fs;
use root_word::RootWord;

fn main() {
  let data                 = fs::read_to_string("data/gismu.txt").expect("err");
  let lines: Vec<&str>     = data.split("\n").collect();
  let words: Vec<RootWord> = lines.iter().filter_map(|line| {
    let value = line.chars().skip(1).take(5).collect();
    RootWord::new(value).ok()
  }).collect();

  words.iter().for_each(|word| {
    println!("{}", word.value)
  })
}
