pub mod value;
pub mod affix;
pub mod source;

use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

use value::Value;
use affix::Affix;
use affix::AffixForm;
use source::Source;

pub struct RootWord {
  pub value:    Value,
  pub meaning:  String,
  pub affixes:  HashMap<AffixForm, Affix>,
  pub sources:  HashSet<Source>
}

impl RootWord {
  pub fn new(value:          Value,
             meaning:        String,
             affix_values:   HashSet<&str>,
             source_values:  HashSet<[&str;3]>) -> RootWord {
    let mut valid_affixes = HashMap::new();
    for affix_value in affix_values {
      if value.valid_affix(affix_value) {
        let affix = Affix::new(affix_value.to_string()).unwrap();
        let form  = Affix::form(affix_value).unwrap();
        valid_affixes.insert(form, affix);
      }
    }
    let mut valid_sources = HashSet::new();
    for source_value_ in source_values {
      let source_value    = source_value_[0];
      let source_source   = source_value_[1];
      let source_language = source_value_[2];
      if value.valid_source(source_value) {
        let source = Source::new(
          source_value.to_string(),
          source_source.to_string(),
          source_language.to_string()
        );
        valid_sources.insert(source);
      }
    }

    RootWord {
      value:    value,
      meaning:  meaning,
      affixes:  valid_affixes,
      sources:  valid_sources,
    }
  }
}

impl fmt::Display for RootWord {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let affixes: Vec<String> = self.affixes.values().map(|affix| {
      affix.to_string()
    }).collect();
    let sources: Vec<String> = self.sources.iter().map(|source| {
      source.to_string()
    }).collect();
    write!(f, "{{\
      \"value\":    {},\
      \"meaning\":  \"{}\",\
      \"affixes\":  [{}],\
      \"sources\":  [{}]\
    }}", self.value, self.meaning, affixes.join(","), sources.join(","))
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
      "".to_string(),
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
      "".to_string(),
      affixes,
      sources,
    );
    assert_eq!(root_word.affixes.keys().len(), 0)
  }
}
