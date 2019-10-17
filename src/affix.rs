#[derive(Debug, PartialEq)]
pub enum AffixForm {
  CVC,
  CCV,
  CVV
}

struct Affix {
}

impl Affix {
  pub fn form(value: &str) -> AffixForm {
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
}
