mod dictionary;
mod effective_force;
mod input;
mod meta;
mod normalization;
mod normalization_value;
mod output;
mod parameter;
mod physics_settings;
mod vector2;
mod vertex;

pub use dictionary::Dictionary;
pub use effective_force::EffectiveForce;
pub use input::Input;
pub use meta::Meta;
pub use normalization::Normalization;
pub use normalization_value::NormalizationValue;
pub use output::Output;
pub use parameter::Parameter;
pub use physics_settings::PhysicsSettings;
pub use vector2::Vector2;
pub use vertex::Vertex;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
#[remain::sorted]
pub struct Physics3 {
  pub meta: Meta,
  pub physics_settings: Vec<PhysicsSettings>,
  pub version: u8,
}
