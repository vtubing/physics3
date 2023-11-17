use crate::Vector2;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Vertex {
  pub acceleration: f64,
  pub delay: f64,
  pub mobility: f64,
  pub position: Vector2,
  pub radius: f64,
}
