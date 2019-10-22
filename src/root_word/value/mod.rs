use std::fmt;
use regex::Regex;
use regex::RegexSet;
use serde_json::json;

pub struct Value {
  pub value:  String,
}

impl Value {
  pub fn new(value: String) -> Option<Value> {
    match Value::valid(&value) {
      false => None,
      true  => Some(Value {
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

  pub fn affix_set(&self) -> RegexSet {
    let cons    = "([bcdfgjklmnprstvxz])";
    let vowel   = "([aeiou])";
    let pattern_cvccv = format!("{}{}{}{}{}", cons, vowel, cons,  cons, vowel);
    let pattern_ccvcv = format!("{}{}{}{}{}", cons, cons,  vowel, cons, vowel);
    let exprsn_cvccv  = Regex::new(&pattern_cvccv).unwrap();
    let exprsn_ccvcv  = Regex::new(&pattern_ccvcv).unwrap();

    let v0 = self.value.chars().nth(0).unwrap();
    let v1 = self.value.chars().nth(1).unwrap();
    let v2 = self.value.chars().nth(2).unwrap();
    let v3 = self.value.chars().nth(3).unwrap();
    let v4 = self.value.chars().nth(4).unwrap();
    if exprsn_cvccv.is_match(&self.value) {
      RegexSet::new(&[
        format!("^{}{}{}$",   v0, v1, v2),
        format!("^{}{}{}$",   v0, v1, v3),
        format!("^{}{}'{}$",  v0, v1, v4),
        format!("^{}{}{}$",   v0, v1, v4),
        format!("^{}{}{}$",   v2, v3, v4),
        format!("^{}{}{}$",   v0, v2, v1)
      ]).unwrap()
    } else if exprsn_ccvcv.is_match(&self.value) {
      RegexSet::new(&[
        format!("^{}{}{}$",   v0, v2, v3),
        format!("^{}{}{}$",   v1, v2, v3),
        format!("^{}{}'{}$",  v0, v2, v4),
        format!("^{}{}{}$",   v0, v2, v4),
        format!("^{}{}'{}$",  v1, v2, v4),
        format!("^{}{}{}$",   v1, v2, v4),
        format!("^{}{}{}$",   v0, v1, v2)
      ]).unwrap()
    } else {
      panic!()
    }
  }
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", json!(self.value))
  }
}

#[cfg(test)]
mod tests {
  use super::Value;

  #[test]
  fn valid() {
    let value = String::from("gleki");
    match Value::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }

  #[test]
  #[should_panic]
  fn invalid() {
    let value = String::from("happy");
    match Value::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }
}
