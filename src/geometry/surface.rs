use crate::linear_algebra::vec4::Vec4;

pub struct Surface {
    weighted_controls: Vec<Vec<Vec4>>,
    knots_u: Vec<f32>,
    knots_v: Vec<f32>,
}

impl Surface {
    pub fn new(weighted_controls: Vec<Vec<Vec4>>, knots_u: Vec<f32>, knots_v: Vec<f32>) -> Surface {
        Surface {
            weighted_controls,
            knots_u,
            knots_v,
        }
    }

    pub fn draw() {
        todo!()
    }
}
