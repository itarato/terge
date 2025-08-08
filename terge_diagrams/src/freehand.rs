use terge::common::U16Point;

pub(crate) struct Freehand {
    pub(crate) points: Vec<U16Point>,
    pub(crate) color: usize,
}

impl Freehand {
    pub(crate) fn new(points: Vec<U16Point>, color: usize) -> Self {
        Self { points, color }
    }
}
