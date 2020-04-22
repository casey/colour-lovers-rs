#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt;
extern crate serde;
pub extern crate reqwest;

mod color;
mod common;
mod error;
mod hsv;
mod module;
mod opt;
mod palette;
mod rgb;
mod api;

use common::*;


fn fetch_color(rgb: Rgb) -> Result<Color, Error> {
  let url = format!("http://www.colourlovers.com/api/color/{}?format=json", rgb); 
  let colors: Vec<Color>  = reqwest::blocking::get(&url)?.json()?; 
  Ok(colors.into_iter().next().unwrap())
}

fn main() -> Result<(), Error> {
  match Opt::from_args() {
    Opt::Color{hex} => println!("{:#?}", fetch_color(hex)?),
    Opt::Palette{id} => {
      let url = format!("http://www.colourlovers.com/api/palette/{}?format=json", id); 
      let palettes: Vec<Palette> = reqwest::blocking::get(&url)?.json()?; 
      let palette = palettes.into_iter().next().unwrap();
      let colors = palette.colors.iter()
        .map(|hex| fetch_color(hex.parse().unwrap()))
        .collect::<Result<Vec<Color>, Error>>()?;
      let module = Module::new(palette, colors)?;
      println!("{}", module);
    }
  }
  Ok(())
}
