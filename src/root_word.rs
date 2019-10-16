use regex::Regex;

struct RootWord {
  value:  String,
}

impl RootWord {
  fn new(value: String) -> Result<RootWord, &'static str> {
    match RootWord::valid(&value) {
      false => Err("Invalid value"),
      true  => Ok(RootWord {
        value
      }),
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
    let exprsn  = Regex::new(&pattern).unwrap();
    exprsn.is_match(&value)
  }
}

#[cfg(test)]
mod tests {
  use super::RootWord;

  #[test]
  fn value() {
    let value = String::from("gleki");
    match RootWord::new(value) {
      Ok(_)   => (),
      Err(_)  => panic!(),
    }
  }

  #[test]
  #[should_panic]
  fn invalid_value() {
    let value = String::from("happy");
    match RootWord::new(value) {
      Ok(_)   => (),
      Err(_)  => panic!(),
    }
  }
}
