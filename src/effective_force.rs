use crate::Vector2;

#[remain::sorted]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct EffectiveForce {
  pub gravity: Vector2,
  pub wind: Vector2,
}
