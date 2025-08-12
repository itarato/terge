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

pub enum TextHorizontalAlign {
    Left,
    Center,
}

pub enum TextVercticalAlign {
    Top,
    Center,
}

pub type I32Point = (i32, i32);
pub type U16Point = (u16, u16);
pub type UsizePoint = (usize, usize);
pub type F32Point = (f32, f32);

pub fn between_u16_inclusive(lhs: u16, rhs: u16, v: u16) -> bool {
    if lhs < rhs {
        (lhs..=rhs).contains(&v)
    } else {
        (rhs..=lhs).contains(&v)
    }
}

pub fn i32point_to_u16point(p: I32Point) -> U16Point {
    (p.0 as u16, p.1 as u16)
}

pub fn u16point_to_i32point(p: U16Point) -> I32Point {
    (p.0 as i32, p.1 as i32)
}

pub fn f32point_to_u16point(p: F32Point) -> U16Point {
    (p.0 as u16, p.1 as u16)
}

/// range must be ordered.
pub fn u16_value_included_in_range(v: u16, range: U16Point) -> bool {
    range.0 <= v && range.1 >= v
}

/// ranges must be ordered
pub fn u16_range_overlap(lhs: U16Point, rhs: U16Point) -> bool {
    !(rhs.1 < lhs.0 || rhs.0 > lhs.1)
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

impl Arithmetics<F32Point, f32> for F32Point {
    fn add(&self, other: F32Point) -> F32Point {
        (self.0 + other.0, self.1 + other.1)
    }

    fn sub(&self, other: F32Point) -> F32Point {
        (self.0 - other.0, self.1 - other.1)
    }

    fn div(&self, divisor: f32) -> F32Point {
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

pub fn multiline_text_line_start(
    line_count: u16,
    line_length: u16,
    line_index: u16,
    start: U16Point,
    horizontal_align: TextHorizontalAlign,
    vertical_align: TextVercticalAlign,
) -> U16Point {
    let x = match horizontal_align {
        TextHorizontalAlign::Left => start.0,
        TextHorizontalAlign::Center => {
            if start.0 < (line_length / 2) {
                0
            } else {
                start.0 - (line_length / 2)
            }
        }
    };
    let y = match vertical_align {
        TextVercticalAlign::Center => {
            if (start.1 + line_index) < (line_count / 2) {
                0
            } else {
                start.1 + line_index - (line_count / 2)
            }
        }
        TextVercticalAlign::Top => start.1 + line_index,
    };

    (x, y)
}

#[derive(Debug)]
pub struct Gravity {
    g: f32,
    max_vy: f32,
}

impl Gravity {
    pub fn new(g: f32, max_vy: f32) -> Self {
        Self { g, max_vy }
    }

    pub fn apply(&self, p: &mut F32Point, v: &mut F32Point) {
        static FLOAT_ZERO_THRESHOLD: f32 = 0.2;

        if v.1 < 0.0 {
            // Raising up.
            v.1 *= 1.0 / self.g;

            // Falling back.
            if v.1.abs() <= FLOAT_ZERO_THRESHOLD {
                v.1 = FLOAT_ZERO_THRESHOLD;
            }
        } else {
            // Falling down.
            v.1 = self.max_vy.min(v.1 * self.g);
        }

        p.1 += v.1;
    }
}
