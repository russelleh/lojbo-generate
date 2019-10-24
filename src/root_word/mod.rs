pub mod value;
pub mod affix;
pub mod source;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use serde_json::json;

use value::Value;
use affix::Affix;
use affix::AffixForm;
use source::Source;

pub struct RootWord {
  pub value:    Value,
  pub affixes:  HashMap<AffixForm, Affix>,
  pub sources:  HashSet<Source>
}

impl RootWord {
  pub fn new(value:          Value,
             affix_values:   HashSet<&str>,
             source_values:  HashSet<&str>) -> RootWord {
    let mut valid_affixes = HashMap::new();
    for affix_value in affix_values {
      if value.valid_affix(affix_value) {
        let affix = Affix::new(affix_value.to_string()).unwrap();
        let form  = Affix::form(affix_value).unwrap();
        valid_affixes.insert(form, affix);
      }
    }
    let mut valid_sources = HashSet::new();
    for source_value in source_values {
      if value.valid_source(source_value) {
        let source = Source::new(source_value.to_string());
        valid_sources.insert(source);
      }
    }

    RootWord {
      value:    value,
      affixes:  valid_affixes,
      sources:  valid_sources,
    }
  }
}

impl fmt::Display for RootWord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let affixes: Vec<String> = self.affixes.values().map(|affix| {
      format!("{}", affix.value)
    }).collect();
    let sources: Vec<String> = self.sources.iter().map(|source| {
      format!("{}", source.value)
    }).collect();
    write!(f, "{}", json!({
      "value":    self.value.value,
      "affixes":  affixes,
      "sources":  sources,
    }))
  }
}

#[cfg(test)]
mod tests {
  use super::HashSet;
  use super::RootWord;
  use super::value::Value;

  #[test]
  fn valid_affixes() {
    let     value   = Value::new("tavla".to_string()).unwrap();
    let mut affixes = HashSet::new();
    let sources = HashSet::new();
    affixes.insert("tav");
    affixes.insert("ta'a");
    let root_word = RootWord::new(
      value,
      affixes,
      sources,
    );
    assert_eq!(root_word.affixes.keys().len(), 2)
  }

  #[test]
  fn invalid_affix() {
    let value   = Value::new("klama".to_string()).unwrap();
    let mut affixes = HashSet::new();
    let sources = HashSet::new();
    affixes.insert("tav");
    let root_word = RootWord::new(
      value,
      affixes,
      sources,
    );
    assert_eq!(root_word.affixes.keys().len(), 0)
  }
}
