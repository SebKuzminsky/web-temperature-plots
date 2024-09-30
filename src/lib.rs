use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy)]
pub struct Stats {
    pub temperatures: [f32; 11],
}

impl Stats {
    pub fn new() -> Self {
        Self {
            temperatures: [
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
                -999_f32,
            ],
        }
    }
}
