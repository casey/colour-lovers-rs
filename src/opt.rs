use crate::common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "colour-lovers")]
pub enum Opt {
  #[structopt(name = "color")]
  Color {
    hex: Rgb,
  },
  #[structopt(name = "palette")]
  Palette {
    id: u64,
  }
}
