use crate::{Dictionary, EffectiveForce};

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Meta {
  pub effective_forces: EffectiveForce,
  pub fps: Option<u64>,
  pub physics_dictionary: Vec<Dictionary>,
  pub physics_setting_count: u64,
  pub total_input_count: u64,
  pub total_output_count: u64,
  pub vertex_count: u64,
}
