use crate::{Rect, line::Line};

pub const BLOCK_CHAR: &'static str = "░";
pub const BOX_TOP_LEFT_CORNER_CHAR: &'static str = "╔";
pub const BOX_TOP_RIGTH_CORNER_CHAR: &'static str = "╗";
pub const BOX_BOTTOM_LEFT_CORNER_CHAR: &'static str = "╚";
pub const BOX_BOTTOM_RIGTH_CORNER_CHAR: &'static str = "╝";
pub const BOX_VERTICAL_CHAR: &'static str = "║";
pub const BOX_HORIZONTAL_CHAR: &'static str = "═";
pub const LINE_CONNECTION_CHAR: &'static str = "X";
pub const DEFAULT_COLOR_CODE: u8 = 0;

pub type I32Point = (i32, i32);
pub type U16Point = (u16, u16);
pub type UsizePoint = (usize, usize);

pub fn i32point_to_u16point(p: I32Point) -> U16Point {
    (p.0 as u16, p.1 as u16)
}

pub fn u16point_to_i32point(p: U16Point) -> I32Point {
    (p.0 as i32, p.1 as i32)
}

pub trait Arithmetics<TPair, T> {
    fn add(&self, other: TPair) -> TPair;
    fn sub(&self, other: TPair) -> TPair;
    fn div(&self, divisor: T) -> TPair;
}

impl Arithmetics<I32Point, i32> for I32Point {
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

impl Arithmetics<U16Point, u16> for U16Point {
    fn add(&self, other: U16Point) -> U16Point {
        (self.0.wrapping_add(other.0), self.1.wrapping_add(other.1))
    }

    fn sub(&self, other: U16Point) -> U16Point {
        (self.0.wrapping_sub(other.0), self.1.wrapping_sub(other.1))
    }

    fn div(&self, divisor: u16) -> U16Point {
        (self.0 / divisor, self.1 / divisor)
    }
}

pub fn point_pair_minmax(lhs: U16Point, rhs: U16Point) -> (u16, u16, u16, u16) {
    (
        lhs.0.min(rhs.0),
        lhs.1.min(rhs.1),
        lhs.0.max(rhs.0),
        lhs.1.max(rhs.1),
    )
}

pub fn intersection_of_rect_and_line(rect: &Rect, line: &Line) -> Vec<U16Point> {
    let slope = line.slope();
    let mut out = vec![];

    if line.y_range().contains(&rect.start.1) {
        let intersect_top_x =
            ((rect.start.1 as f32 - line.start.1 as f32) / slope) + line.start.0 as f32;
        if intersect_top_x >= rect.start.0 as f32 && intersect_top_x <= rect.end().0 as f32 {
            out.push((intersect_top_x.round() as u16, rect.start.1));
        }
    }

    if line.y_range().contains(&(rect.start.1 + rect.size.1)) {
        let intersect_bottom_x = ((rect.start.1 as f32 + rect.size.1 as f32 - line.start.1 as f32)
            / slope)
            + line.start.0 as f32;
        if intersect_bottom_x >= rect.start.0 as f32 && intersect_bottom_x <= rect.end().0 as f32 {
            out.push((
                intersect_bottom_x.round() as u16,
                rect.start.1 + rect.size.1,
            ));
        }
    }

    if line.x_range().contains(&rect.start.0) {
        let intersect_left_y =
            (slope * (rect.start.0 as f32 - line.start.0 as f32)) + line.start.1 as f32;
        if intersect_left_y >= rect.start.1 as f32 && intersect_left_y <= rect.end().1 as f32 {
            out.push((rect.start.0, intersect_left_y.round() as u16));
        }
    }

    if line.x_range().contains(&(rect.start.0 + rect.size.0)) {
        let intersect_right_y = (slope
            * (rect.start.0 as f32 + rect.size.0 as f32 - line.start.0 as f32))
            + line.start.1 as f32;
        if intersect_right_y >= rect.start.1 as f32 && intersect_right_y <= rect.end().1 as f32 {
            out.push((rect.start.0 + rect.size.0, intersect_right_y.round() as u16));
        }
    }

    out
}
