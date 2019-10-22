mod root_word;

use std::fs;

use root_word::RootWord;
use root_word::value::Value;
use root_word::affix::Affix;

fn main() {
  let data:   String    = fs::read_to_string("data/gismu.txt").expect("err");
  let head:   Vec<&str> = data.split("\n").skip(1).collect();
  let tail:   usize     = head.len() - 1;
  let lines:  Vec<&str> = head.into_iter().take(tail).collect();

  for line in lines {
    let value_value:  String
      = line.chars().skip(1).take(5).collect();
    let value_option: Option<Value> = Value::new(value_value);
    if value_option.is_some() {
      let value:              Value   = value_option.unwrap();
      let affix_value_tail_0: String  = line.chars().skip(7 ).take(4).collect();
      let affix_value_tail_1: String  = line.chars().skip(11).take(4).collect();
      let affix_value_tail_2: String  = line.chars().skip(15).take(5).collect();
      let affix_value_0               = String::from(affix_value_tail_0.trim());
      let affix_value_1               = String::from(affix_value_tail_1.trim());
      let affix_value_2               = String::from(affix_value_tail_2.trim());
      let affix_options_long: Vec<Option<Affix>>  = vec![
        Affix::new(affix_value_0),
        Affix::new(affix_value_1),
        Affix::new(affix_value_2)
      ];
      let affix_options:      Vec<Option<Affix>>
        = affix_options_long.into_iter().filter(|affix| {
        affix.is_some()
      }).collect();
      let affixes:            Vec<Affix>
        = affix_options.into_iter().map(|affix| {
        match affix {
          Some(a) => a,
          None    => panic!()
        }
      }).collect();

      let root_word = RootWord::new(value, affixes);

      println!("{}", root_word);
    }
  }
}
