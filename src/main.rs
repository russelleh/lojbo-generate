mod root_word;

use std::fs;
use root_word::RootWord;
use root_word::value::Value;

fn main() {
  let data = fs::read_to_string("data/gismu.txt").expect("err");

  let lines:  Vec<&str>     = data.split("\n").collect();
  let words:  Vec<RootWord> = lines.iter().filter_map(|line| {
    Some(RootWord::new(
      Value::new(
        line.chars().skip(1).take(5).collect()
      ).ok().unwrap(),
      vec![]
    ))
  }).collect();
  words.iter().for_each(|word| {
    println!("{}", word.value.value)
  })
}
