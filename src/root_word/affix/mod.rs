use std::fmt;
use regex::Regex;

#[derive(Debug, PartialEq, Hash, Eq)]
pub enum AffixForm {
  CVC,
  CCV,
  CVV
}

#[derive(PartialEq, Hash, Eq)]
pub struct Affix {
  pub value:  String,
}

impl Affix {
  pub fn new(value: String) -> Option<Affix> {
    match Affix::valid(&value) {
      false => None,
      true  => Some(Affix {
        value,
      })
    }
  }

  fn valid(value: &str) -> bool {
    let pair      = Regex::new("([bcfgkmpsvx][lr]|[td]r|[cs][pftkmn]|\
      [jz][bvdgm]|t[cs]|d[jz])").unwrap();
    let diphthong = Regex::new("('|[aeo]i|au)").unwrap();
    match Affix::form(value) {
      None              => false,
      Some(affix_form)  => match affix_form {
        AffixForm::CVC  => true,
        AffixForm::CCV  => pair.is_match(value),
        AffixForm::CVV  => diphthong.is_match(value)
      }
    }
  }

  pub fn form(value: &str) -> Option<AffixForm> {
    let cons    = "([bcdfgjklmnprstvxz])";
    let vowel   = "([aeiou])";
    let pattern_cvc = format!("^{}{}{}$",   cons, vowel, cons);
    let pattern_ccv = format!("^{}{}{}$",   cons, cons,  vowel);
    let pattern_cvv = format!("^{}{}'?{}$", cons, vowel, vowel);
    let expr_cvc    = regex::Regex::new(&pattern_cvc).unwrap();
    let expr_ccv    = regex::Regex::new(&pattern_ccv).unwrap();
    let expr_cvv    = regex::Regex::new(&pattern_cvv).unwrap();

    if expr_cvc.is_match(value) {
      Some(AffixForm::CVC)
    } else if expr_ccv.is_match(value) {
      Some(AffixForm::CCV)
    } else if expr_cvv.is_match(value) {
      Some(AffixForm::CVV)
    } else {
      None
    }
  }
}

impl fmt::Display for Affix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.value)
  }
}

#[cfg(test)]
mod tests {
  use super::AffixForm;
  use super::Affix;

  #[test]
  fn form_cvc() {
    let form    = Affix::form("tav").unwrap();
    assert_eq!(AffixForm::CVC, form)
  }

  #[test]
  fn form_ccv() {
    let form    = Affix::form("vla").unwrap();
    assert_eq!(AffixForm::CCV, form)
  }

  #[test]
  fn form_cvv() {
    let form    = Affix::form("tau").unwrap();
    assert_eq!(AffixForm::CVV, form)
  }

  #[test]
  fn form_cvhv() {
    let form    = Affix::form("ta'a").unwrap();
    assert_eq!(AffixForm::CVV, form)
  }

  #[test]
  fn valid() {
    let value = String::from("tav");
    match Affix::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }

  #[test]
  fn valid_diphthong() {
    let value = String::from("toi");
    match Affix::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }

  #[test]
  #[should_panic]
  fn invalid_diphthong() {
    let value = String::from("tio");
    match Affix::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }

  #[test]
  fn valid_initial_pair() {
    let value = String::from("zma");
    match Affix::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }

  #[test]
  #[should_panic]
  fn invalid_initial_pair() {
    let value = String::from("zna");
    match Affix::new(value) {
      Some(_) => (),
      None    => panic!()
    }
  }
}
