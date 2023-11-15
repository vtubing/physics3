#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Physics3 {
  pub meta: Meta,
  pub physics_settings: Vec<PhysicsSettings>,
  pub version: u8,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Dictionary {
  pub id: String,
  pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Normalization {
  pub angle: NormalizationValue,
  pub position: NormalizationValue,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Vector2 {
  pub x: f64,
  pub y: f64,
}

#[remain::sorted]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct EffectiveForce {
  pub gravity: Vector2,
  pub wind: Vector2,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct NormalizationValue {
  pub default: f64,
  pub maximum: f64,
  pub minimum: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Parameter {
  pub id: String,
  pub target: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
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
