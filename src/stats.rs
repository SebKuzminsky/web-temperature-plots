#[derive(serde::Deserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct Stats {
    pub temperatures: [f32; 11],
}
