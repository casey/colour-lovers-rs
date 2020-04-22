use crate::common::*;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Color {
  #[serde(rename = "userName")]
  pub username:     String,
  pub title:        String,
  pub api_url:      String,
  pub badge_url:    String,
  pub date_created: String,
  pub description:  String,
  pub hex:          String,
  pub id:           u64,
  pub image_url:    String,
  pub num_comments: u64,
  pub num_hearts:   f64,
  pub num_views:    u64,
  pub num_votes:    u64,
  pub rank:         u64,
  pub url:          String,
  pub hsv:          Hsv,
  pub rgb:          Rgb,
}
