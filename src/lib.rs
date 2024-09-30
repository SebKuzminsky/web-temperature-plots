use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct Stats {
    pub temperatures: [f32; 11],
}
