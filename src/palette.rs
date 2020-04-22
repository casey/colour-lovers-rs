use crate::common::*;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Palette {
  #[serde(rename = "userName")]
  pub username:     String,
  pub id:           u64,
  pub title:        String,
  pub api_url:      String,
  pub badge_url:    String,
  pub date_created: String,
  pub description:  String,
  pub image_url:    String,
  pub num_comments: u64,
  pub num_hearts:   f64,
  pub num_views:    u64,
  pub num_votes:    u64,
  pub rank:         u64,
  pub url:          String,
  pub colors:       Vec<String>,
}
