pub mod value;
pub mod affix;

use std::collections::HashMap;

use value::Value;
use affix::Affix;
use affix::AffixForm;

pub struct RootWord {
  pub value:    Value,
  pub affixes:  HashMap<AffixForm, Affix>
}

impl RootWord {
  pub fn new(value: Value, affixes: Vec<Affix>) -> RootWord {
    let mut valid_affixes = HashMap::new();
    for affix in affixes {
      if value.affix_set().is_match(&affix.value) {
        valid_affixes.insert(Affix::form(&affix.value), affix);
      }
    }
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
    assert_eq!(root_word.affixes.keys().len(), 2)
  }

  #[test]
  fn invalid_affix() {
    let root_word = RootWord::new(
      Value::new(String::from("klama")).ok().unwrap(),
      vec![
        Affix::new(String::from("tav")).ok().unwrap(),
      ]
    );
    assert_eq!(root_word.affixes.keys().len(), 0)
  }
}
