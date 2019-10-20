use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum AffixForm {
  CVC,
  CCV,
  CVV
}

struct Affix {
  value:  String,
}

impl Affix {
  fn new(value: String) -> Result<Affix, ()> {
    match Affix::valid(&value) {
      false => Err(()),
      true  => Ok(Affix {
        value,
      })
    }
  }

  fn valid(value: &str) -> bool {
    let pair      = Regex::new("([bcfgkmpsvx][lr]|[td]r|[cs][pftkmn]|\
      [jz][bvdgm]|t[cs]|d[jz])").unwrap();
    let diphthong = Regex::new("('|[aeo]i|au)").unwrap();
    match Affix::form(value) {
      AffixForm::CCV  => pair.is_match(value),
      AffixForm::CVV  => diphthong.is_match(value),
      _               => true
    }
  }

  fn form(value: &str) -> AffixForm {
    let cons    = "([bcdfgjklmnprstvxz])";
    let vowel   = "([aeiou])";
    let pattern_cvc = format!("^{}{}{}$",   cons, vowel, cons);
    let pattern_ccv = format!("^{}{}{}$",   cons, cons,  vowel);
    let pattern_cvv = format!("^{}{}'?{}$", cons, vowel, vowel);
    let expr_cvc    = regex::Regex::new(&pattern_cvc).unwrap();
    let expr_ccv    = regex::Regex::new(&pattern_ccv).unwrap();
    let expr_cvv    = regex::Regex::new(&pattern_cvv).unwrap();

    if expr_cvc.is_match(value) {
      AffixForm::CVC
    } else if expr_ccv.is_match(value) {
      AffixForm::CCV
    } else if expr_cvv.is_match(value) {
      AffixForm::CVV
    } else {
      panic!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AffixForm;
  use super::Affix;

  #[test]
  fn form_cvc() {
    let form    = Affix::form("tav");
    assert_eq!(AffixForm::CVC, form)
  }

  #[test]
  fn form_ccv() {
    let form    = Affix::form("vla");
    assert_eq!(AffixForm::CCV, form)
  }

  #[test]
  fn form_cvv() {
    let form    = Affix::form("tau");
    assert_eq!(AffixForm::CVV, form)
  }

  #[test]
  fn form_cvhv() {
    let form    = Affix::form("ta'a");
    assert_eq!(AffixForm::CVV, form)
  }

  #[test]
  fn valid() {
    let value = String::from("tav");
    match Affix::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }

  #[test]
  fn valid_diphthong() {
    let value = String::from("toi");
    match Affix::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }

  #[test]
  #[should_panic]
  fn invalid_diphthong() {
    let value = String::from("tio");
    match Affix::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }

  #[test]
  fn valid_initial_pair() {
    let value = String::from("zma");
    match Affix::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }

  #[test]
  #[should_panic]
  fn invalid_initial_pair() {
    let value = String::from("zna");
    match Affix::new(value) {
      Err(_)  => panic!(),
      _       => ()
    }
  }
}
