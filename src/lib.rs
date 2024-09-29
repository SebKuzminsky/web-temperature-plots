use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, )]
pub struct Stats {
    pub temperatures: [f32; 11],
}
