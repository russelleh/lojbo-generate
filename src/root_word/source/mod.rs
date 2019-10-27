use std::fmt;

#[derive(Eq, Hash, PartialEq)]
pub enum Language {
    En,
    Es,
    Hi,
    Ru,
    Zh
}

#[derive(Eq, Hash, PartialEq)]
pub struct Source {
  pub value:    String,
  pub source:   String,
  pub language: Language,
}

impl Source {
  pub fn new(value: String, source: String, l: String) -> Source {
    let language = Source::language(&l);
    Source {
      value,
      source,
      language
    }
  }

  pub fn language_key(&self) -> &str {
    match self.language {
      Language::En => "en",
      Language::Es => "es",
      Language::Hi => "hi",
      Language::Ru => "ru",
      Language::Zh => "zh"
    }
  }

  pub fn language(key: &str) -> Language {
    match key {
      "en"  => Language::En,
      "es"  => Language::Es,
      "hi"  => Language::Hi,
      "ru"  => Language::Ru,
      "zh"  => Language::Zh,
      _     => panic!()
    }
  }
}

impl fmt::Display for Source {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{
      \"value\":    \"{}\",\
      \"source\":   \"{}\",\
      \"language\": \"{}\"\
    }}", self.value, self.source, self.language_key())
  }
}
