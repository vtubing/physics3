use crate::Parameter;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Input {
  pub reflect: bool,
  pub source: Parameter,
  #[serde(rename = "Type")]
  pub type_: String,
  pub weight: usize,
}
