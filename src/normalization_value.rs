#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct NormalizationValue {
  pub default: f64,
  pub maximum: f64,
  pub minimum: f64,
}
