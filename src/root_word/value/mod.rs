use std::fmt;
use regex::Regex;
use serde_json::json;

pub struct Value {
  pub value:  String,
}

impl Value {
  pub fn new(value: String) -> Result<Value, ()> {
    match Value::valid(&value) {
      false => Err(()),
      true  => Ok(Value {
        value
      })
    }
  }

  fn valid(value: &str) -> bool {
    let cons    = "([bcdfgjklmnprstvxz])";
    let vowel   = "([aeiou])";
    let pair    = concat!("([bcfgkmpsvx][lr]|[td]r|[cs][pftkmn]|[jz][bvdgm]|",
      "t[cs]|d[jz])");
    let cluster = concat!("([bdgjvzcfkpstx][lrmn]|[lrn][bdgjvzcfkpstx]|",
      "b[dgjvz]|d[bgjvz]|g[bdjvz]|j[bdgv]|v[bdgjz]|z[bdgv]|c[fkpt]|f[ckpstx]|",
      "k[cfpst]|p[cfkstx]|s[fkptx]|t[cfkpsx]|x[fpst]|l[rmn]|r[lmn]|",
      "m[lrnbdgjvcfkpstx]|n[lrm])");
    let pattern = format!("^({}{}{}|{}{}{}){}$", pair, vowel, cons, cons, vowel,
      cluster, vowel);
    Regex::new(&pattern).unwrap().is_match(&value)
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", json!({
      "value":  self.value,
    }))
  }
}

#[cfg(test)]
mod tests {
  use super::Value;

  #[test]
  fn valid() {
    let value = String::from("gleki");
    match Value::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }

  #[test]
  #[should_panic]
  fn invalid() {
    let value = String::from("happy");
    match Value::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }
}
