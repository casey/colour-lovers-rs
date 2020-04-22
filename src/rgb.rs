use crate::common::*;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rgb {
  pub red:   u8,
  pub green: u8,
  pub blue:  u8,
}

impl Display for Rgb {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
  }
}


#[derive(Debug)]
pub enum RgbError {
  Length{length: usize},
  Digit{digit: char},
}

impl Display for RgbError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::RgbError::*;
    match self {
      Length{length} => write!(f, "bad length: {}: rgb values must be six hex digits", length),
      Digit{digit}   => write!(f, "bad digit: {:?}: rgb valuse must be in hexidecimal", digit ),
    }
  }
}

impl FromStr for Rgb {
  type Err = RgbError;

  fn from_str(s: &str) -> Result<Rgb, RgbError> {
    let length = s.len();

    if length != 6 {
      return Err(RgbError::Length{length});
    }

    for digit in s.chars() {
      match digit {
        '0'...'9' |
        'a'...'z' |
        'A'...'Z' => {},
        _ => return Err(RgbError::Digit{digit}),
      }
    }

    let red   = u8::from_str_radix(&s[0..2], 16).unwrap();
    let green = u8::from_str_radix(&s[2..4], 16).unwrap();
    let blue  = u8::from_str_radix(&s[4..6], 16).unwrap();
   
    Ok(Rgb {
      red,
      green,
      blue,
    })
  }
}
