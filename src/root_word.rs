struct RootWord {
  value:  String,
}

impl RootWord {
  fn new(value_: &str) -> RootWord {
    let value = String::from(value_);
    RootWord {
      value,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::RootWord;

  #[test]
  fn value() {
    let root_word = RootWord::new("gleki");
  }
}
