use crate::common::*;

pub struct Module {
  name:   String,
  colors: Vec<(String, Rgba)>,
}

struct Rgba(Rgb);

impl Display for Rgba {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let Rgb{red, green, blue} = self.0;
    write!(f, "Color{{r: 0x{:02X}, g: 0x{:02X}, b: 0x{:02X}, a: 0xFF}}", red, green, blue)
  }
}

impl Module {
  pub fn new(palette: Palette, colors: Vec<Color>) -> Result<Module, Error> {
    let name = palette.title.to_lowercase()
      .chars()
      .map(|c| if c == ' ' {
        '_'
      } else {
        c
      })
     .collect::<String>();

    let colors = colors.into_iter()
      .map(|color| {
        let title = color.title.to_uppercase()
          .chars()
          .map(|c| if c == ' ' {
            '_'
          } else {
            c
          })
          .collect();

        (title, Rgba(color.rgb))
      })
      .collect();

    Ok(Module {
      name,
      colors,
    })
  }
}

impl Display for Module {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let width = self.colors.iter()
      .map(|(name, _)| name.len())
      .max()
      .unwrap();

    let palette = self.colors.iter()
      .map(|(name, _)| name.clone())
      .collect::<Vec<String>>()
      .join(",");
    writeln!(f, "mod {} {{", self.name)?;
    writeln!(f, "  pub const PALETTE: &[Color] = &[{}];", palette)?;
    for (name, rgba) in &self.colors {
      writeln!(f, "  pub const {}:{: <width$} Color = {};", name, "", rgba, width = width - name.len())?;
    }
    write!(f, "}}")
  }
}

