use std::ops;

use crate::common::*;

pub struct LinePointsIterator {
    x_max: i32,
    y_max: i32,
    diff_x: f32,
    diff_y: f32,
    start: I32Point,
    i: i32,
}

impl LinePointsIterator {
    pub fn new(lhs: I32Point, rhs: I32Point) -> Self {
        let (x_min, y_min, x_max, y_max) = point_pair_minmax(lhs, rhs);

        let diff_x = (rhs.0 - lhs.0) as f32;
        let diff_y = (rhs.1 - lhs.1) as f32;

        let i = if diff_x.abs() >= diff_y.abs() {
            x_min
        } else {
            y_min
        };

        Self {
            x_max,
            y_max,
            diff_x,
            diff_y,
            start: lhs,
            i,
        }
    }
}

impl Iterator for LinePointsIterator {
    type Item = I32Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.diff_x.abs() >= self.diff_y.abs() {
            if self.diff_x != 0.0 {
                if self.i > self.x_max {
                    return None;
                }

                let x = self.i;
                self.i += 1;

                let y = ((self.diff_y / self.diff_x) * (x as f32 - self.start.0 as f32)
                    + self.start.1 as f32)
                    .round() as usize;

                return Some((x, y as i32));
            }
        } else {
            if self.diff_y != 0.0 {
                if self.i > self.y_max {
                    return None;
                }

                let y = self.i;
                self.i += 1;

                let x = ((self.diff_x / self.diff_y) * (y as f32 - self.start.1 as f32)
                    + self.start.0 as f32)
                    .round() as usize;
                return Some((x as i32, y));
            }
        }

        None
    }
}

#[derive(Debug)]
pub struct Line {
    pub start: I32Point,
    pub end: I32Point,
}

impl Line {
    pub fn slope(&self) -> f32 {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        dy as f32 / dx as f32
    }

    pub fn x_range(&self) -> ops::RangeInclusive<i32> {
        self.start.0.min(self.end.0)..=self.start.0.max(self.end.0)
    }

    pub fn y_range(&self) -> ops::RangeInclusive<i32> {
        self.start.1.min(self.end.1)..=self.start.1.max(self.end.1)
    }

    pub fn is_point_on(&self, p: I32Point) -> bool {
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
