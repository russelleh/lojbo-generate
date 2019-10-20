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
    let valid_affixes = affixes.into_iter().filter(|affix| {
      value.affix_set().is_match(&affix.value)
    }).collect();
    RootWord {
      value:    value,
      affixes:  valid_affixes
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
    let root_word = RootWord::new(
      Value::new(String::from("tavla")).ok().unwrap(),
      vec![
        Affix::new(String::from("tav")).ok().unwrap(),
        Affix::new(String::from("ta'a")).ok().unwrap()
      ]
    );
    assert_eq!(root_word.affixes.len(), 2)
  }

  #[test]
  fn invalid_affix() {
    let root_word = RootWord::new(
      Value::new(String::from("klama")).ok().unwrap(),
      vec![
        Affix::new(String::from("tav")).ok().unwrap(),
      ]
    );
    assert_eq!(root_word.affixes.len(), 0)
  }
}
