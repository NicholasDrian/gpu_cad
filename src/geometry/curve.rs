pub struct Curve {
    weighted_controls: Vec<Vec4>,
    knots: Vec<f32>,
}

impl Curve {
    pub fn new(weighted_controls: Vec<Vec4>, knots: Vec<f32>) -> Curve {
        Curve {
            weighted_controls,
            knots,
        }
    }

    pub fn draw() {
        todo!()
    }
}
