#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(deny_unknown_fields)]
pub struct Hsv {
  hue:        u8,
  saturation: u8,
  value:      u8,
}
