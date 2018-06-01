use common::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "iris")]
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
