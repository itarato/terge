use crate::{Rect, line::Line};

pub const BLOCK_CHAR: &'static str = "█";
pub const DEFAULT_COLOR_CODE: u8 = 0;

pub type I32Point = (i32, i32);
pub type UsizePoint = (usize, usize);

pub trait Arithmetics<T> {
    fn add(&self, other: T) -> T;
    fn sub(&self, other: T) -> T;
    fn div(&self, divisor: i32) -> T;
}

impl Arithmetics<I32Point> for I32Point {
    fn add(&self, other: I32Point) -> I32Point {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: I32Point) -> I32Point {
        (self.0 - other.0, self.1 - other.1)
    }

    fn div(&self, divisor: i32) -> I32Point {
        (self.0 / divisor, self.1 / divisor)
    }
}

pub fn point_pair_minmax(lhs: I32Point, rhs: I32Point) -> (i32, i32, i32, i32) {
    (
        lhs.0.min(rhs.0),
        lhs.1.min(rhs.1),
        lhs.0.max(rhs.0),
        lhs.1.max(rhs.1),
    )
}

pub fn intersection_of_rect_and_line(rect: &Rect, line: &Line) -> Vec<I32Point> {
    let slope = line.slope();
    let mut out = vec![];

    if line.y_range().contains(&rect.start.1) {
        let intersect_top_x =
            (((rect.start.1 - line.start.1) as f32 / slope) + line.start.0 as f32).round() as i32;
        if intersect_top_x >= rect.start.0 && intersect_top_x <= rect.start.0 + rect.size.0 {
            out.push((intersect_top_x, rect.start.1));
        }
    }

    if line.y_range().contains(&(rect.start.1 + rect.size.1)) {
        let intersect_bottom_x = (((rect.start.1 + rect.size.1 - line.start.1) as f32 / slope)
            + line.start.0 as f32)
            .round() as i32;
        if intersect_bottom_x >= rect.start.0 && intersect_bottom_x <= rect.start.0 + rect.size.0 {
            out.push((intersect_bottom_x, rect.start.1 + rect.size.1));
        }
    }

    if line.x_range().contains(&rect.start.0) {
        let intersect_left_y =
            ((slope * (rect.start.0 - line.start.0) as f32) + line.start.1 as f32).round() as i32;
        if intersect_left_y >= rect.start.1 && intersect_left_y <= rect.start.1 + rect.size.1 {
            out.push((rect.start.0, intersect_left_y));
        }
    }

    if line.x_range().contains(&(rect.start.0 + rect.size.0)) {
        let intersect_right_y = ((slope * (rect.start.0 + rect.size.0 - line.start.0) as f32)
            + line.start.1 as f32)
            .round() as i32;
        if intersect_right_y >= rect.start.1 && intersect_right_y <= rect.start.1 + rect.size.1 {
            out.push((rect.start.0 + rect.size.0, intersect_right_y));
        }
    }

    out
}
