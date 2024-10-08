use serde;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct Stats {
    // This is the temperatures reported in
    // `/sys/devices/virtual/thermal/thermal_zone*`.
    pub temperatures: Vec<f32>,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            temperatures: vec![],
        }
    }
}
