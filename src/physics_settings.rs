use crate::{Input, Normalization, Output, Vertex};

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct PhysicsSettings {
  pub id: String,
  pub input: Vec<Input>,
  pub normalization: Normalization,
  pub output: Vec<Output>,
  pub vertices: Vec<Vertex>,
}
