use crate::common::*;

pub struct Rect {
    // Start is always the upper-left corner (min-x:min-y).
    pub start: U16Point,
    // Size is always positive.
    pub size: U16Point,
}

impl Rect {
    pub fn new_from_unordered_points(lhs: U16Point, rhs: U16Point) -> Self {
        let (min_x, min_y, max_x, max_y) = point_pair_minmax(lhs, rhs);
        Self {
            start: (min_x, min_y),
            size: (max_x - min_x, max_y - min_y),
        }
    }

    pub fn is_point_on_header(&self, p: U16Point) -> bool {
        p.1 == self.start.1 && p.0 >= self.start.0 && p.0 <= (self.start.0 + self.size.0)
    }

    pub fn is_point_on(&self, p: U16Point) -> bool {
        p.0 >= self.start.0
            && p.0 <= (self.start.0 + self.size.0)
            && p.1 >= self.start.1
            && p.1 <= (self.start.1 + self.size.1)
    }

    pub fn is_point_inside(&self, p: U16Point) -> bool {
        p.0 >= self.start.0 + 1
            && p.0 <= (self.start.0 + self.size.0) - 1
            && p.1 >= self.start.1 + 1
            && p.1 <= (self.start.1 + self.size.1) - 1
    }

    pub fn midpoint(&self) -> U16Point {
        self.start.add(self.size.div(2))
    }

    pub fn end(&self) -> U16Point {
        self.start.add(self.size)
    }

    pub fn area(&self) -> u16 {
        self.size.0 * self.size.1
    }
}
