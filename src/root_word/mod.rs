mod value;
mod affix;

use value::Value;

pub struct RootWord {
  pub value:  Value
}

impl RootWord {
  pub fn new(v: String) -> Result<RootWord, ()> {
    match Value::new(v) {
      Err(_)    => Err(()),
      Ok(value) => Ok(RootWord {
        value
      })
    }
  }
}
