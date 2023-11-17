use crate::Parameter;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Output {
  pub destination: Parameter,
  pub reflect: bool,
  pub scale: f64,
  #[serde(rename = "Type")]
  pub type_: String,
  pub vertex_index: usize,
  pub weight: usize,
}
