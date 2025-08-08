use std::ops;

use crate::common::*;

pub struct LinePointsIterator {
    diff_x: f32,
    diff_y: f32,
    i: u16,
    lhs: U16Point,
    rhs: U16Point,
}

impl LinePointsIterator {
    pub fn new(lhs: U16Point, rhs: U16Point) -> Self {
        let diff_x = (rhs.0 as i32 - lhs.0 as i32) as f32;
        let diff_y = (rhs.1 as i32 - lhs.1 as i32) as f32;

        let i = if diff_x.abs() >= diff_y.abs() {
            lhs.0
        } else {
            lhs.1
        };

        Self {
            diff_x,
            diff_y,
            i,
            lhs,
            rhs,
        }
    }
}

impl Iterator for LinePointsIterator {
    type Item = U16Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.diff_x.abs() >= self.diff_y.abs() {
            if self.diff_x != 0.0 {
                if !between_u16_inclusive(self.lhs.0, self.rhs.0, self.i) {
                    return None;
                }

                let x = self.i;

                if self.lhs.0 < self.rhs.0 {
                    self.i += 1;
                } else {
                    if self.i == 0 {
                        return None;
                    }
                    self.i -= 1;
                }

                let y = ((self.diff_y / self.diff_x) * (x as f32 - self.lhs.0 as f32)
                    + self.lhs.1 as f32)
                    .round() as u16;

                return Some((x, y));
            }
        } else {
            if self.diff_y != 0.0 {
                if !between_u16_inclusive(self.lhs.1, self.rhs.1, self.i) {
                    return None;
                }

                let y = self.i;
                if self.lhs.1 < self.rhs.1 {
                    self.i += 1;
                } else {
                    if self.i == 0 {
                        return None;
                    }
                    self.i -= 1;
                }

                let x = ((self.diff_x / self.diff_y) * (y as f32 - self.lhs.1 as f32)
                    + self.lhs.0 as f32)
                    .round() as u16;
                return Some((x, y));
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub start: U16Point,
    pub end: U16Point,
}

impl Line {
    pub fn slope(&self) -> f32 {
        let dx = self.end.0 as i32 - self.start.0 as i32;
        let dy = self.end.1 as i32 - self.start.1 as i32;
        dy as f32 / dx as f32
    }

    pub fn x_range(&self) -> ops::RangeInclusive<u16> {
        self.start.0.min(self.end.0)..=self.start.0.max(self.end.0)
    }

    pub fn y_range(&self) -> ops::RangeInclusive<u16> {
        self.start.1.min(self.end.1)..=self.start.1.max(self.end.1)
    }

    pub fn is_point_on(&self, p: U16Point) -> bool {
        for line_point in self.iter() {
            if line_point == p {
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> LinePointsIterator {
        LinePointsIterator::new(self.start, self.end)
    }
}
