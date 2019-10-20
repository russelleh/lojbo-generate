pub mod value;
pub mod affix;

use value::Value;
use affix::Affix;

pub struct RootWord {
  pub value:    Value,
  pub affixes:  Vec<Affix>
}

impl RootWord {
  pub fn new(value: Value, affixes: Vec<Affix>) -> RootWord {
    RootWord {
      value,
      affixes
    }
  }
}

#[cfg(test)]
mod tests {
  use super::RootWord;
  use super::value::Value;
  use super::affix::Affix;

  #[test]
  fn valid_affixes() {
    let _root_word = RootWord::new(
      Value::new(String::from("tavla")).ok().unwrap(),
      vec![
        Affix::new(String::from("tav")).ok().unwrap(),
        Affix::new(String::from("ta'a")).ok().unwrap()
      ]
    );
  }
}
