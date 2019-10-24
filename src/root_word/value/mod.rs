use std::fmt;
use regex::Regex;
use regex::RegexSet;
use serde_json::json;

pub struct Value {
  pub value:  String,
}

impl Value {
  pub fn new(value: String) -> Option<Value> {
    match Value::valid_value(&value) {
      false => None,
      true  => Some(Value {
        value
      })
    }
  }

  fn valid_value(value: &str) -> bool {
    Value::valid_values().is_match(&value)
  }

  pub fn valid_affix(&self, affix_value: &str) -> bool {
    self.valid_affixes().is_match(affix_value)
  }

  pub fn valid_source(&self, source_value: &str) -> bool {
    Value::score(0, &self.value, source_value) >= 2
  }

  fn valid_values() -> Regex {
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
    Regex::new(&pattern).unwrap()
  }

  pub fn valid_affixes(&self) -> RegexSet {
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

  fn score(acc: usize, value: &str, source: &str) -> usize {
    if value == "" {
      acc
    } else if source == "" {
      acc
    } else {
      let value_head:   String  = value.chars().take(1).collect();
      let source_head:  String  = source.chars().take(1).collect();
      if value_head != source_head {
        acc
      } else {
        let value_tail:   String  = value.chars().skip(1).collect();
        let source_tail:  String  = source.chars().skip(1).collect();
        let value_next:   String  = value_tail.chars().skip(1).collect();
        let source_next:  String  = source_tail.chars().skip(1).collect();
        let score_a               = Value::score(acc, &value_tail, &source_tail);
        let score_b               = Value::score(acc, &value_next, &source_tail);
        let score_c               = Value::score(acc, &value_tail, &source_next);
        let scores                = vec![score_a, score_b, score_c];
        let score_max             = scores.iter().max().unwrap();
        acc + 1 + score_max
      }
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

  #[test]
  fn valid_source() {
    let value = String::from("katna");
    match Value::new(value) {
      Some(v) => assert!(v.valid_source("kan")),
      None    => panic!()
    }
  }

  #[test]
  #[should_panic]
  fn invalid_source() {
    let value = String::from("katna");
    match Value::new(value) {
      Some(v) => assert!(v.valid_source("kort")),
      None    => panic!()
    }
  }
}
