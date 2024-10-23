use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Plot {
    pub canvas: NodeRef,
    pub data: Vec<(f32, f32)>,
    // Optional index of right-hand side of the plot, follow new data
    // if None.
    pub x_max: Option<usize>,
}

impl Plot {
    pub fn default() -> Self {
        Self {
            canvas: NodeRef::default(),
            data: vec![],
            x_max: None,
        }
    }
}
