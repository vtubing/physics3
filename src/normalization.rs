use crate::NormalizationValue;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Normalization {
  pub angle: NormalizationValue,
  pub position: NormalizationValue,
}
