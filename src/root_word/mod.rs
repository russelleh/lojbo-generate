pub mod value;
pub mod affix;

use std::fmt;
use std::collections::HashMap;
use serde_json::json;

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
        valid_affixes.insert(Affix::form(&affix.value).unwrap(), affix);
      }
    }
    RootWord {
      value:    value,
      affixes:  valid_affixes
    }
  }
}

impl fmt::Display for RootWord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let affix_values: Vec<String> = self.affixes.values().into_iter().map(|affix| {
      format!("{}", affix)
    }).collect();
    write!(f, "{}", json!({
      "value":    self.value.value,
      "affixes":  affix_values
    }))
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
      Value::new(String::from("tavla")).unwrap(),
      vec![
        Affix::new(String::from("tav")).unwrap(),
        Affix::new(String::from("ta'a")).unwrap()
      ]
    );
    assert_eq!(root_word.affixes.keys().len(), 2)
  }

  #[test]
  fn invalid_affix() {
    let root_word = RootWord::new(
      Value::new(String::from("klama")).unwrap(),
      vec![
        Affix::new(String::from("tav")).unwrap(),
      ]
    );
    assert_eq!(root_word.affixes.keys().len(), 0)
  }
}
