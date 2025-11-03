// Copyright 2020 Google LLC
// Copyright 2023 Martin Pool
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Simple 2D integer-indexed point.
use std::cmp::{max, min};
use std::fmt;

use strum_macros::EnumIter;

#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "point({}, {})", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

/// Shorthand to construct a point.
#[must_use]
pub const fn point(x: isize, y: isize) -> Point {
    Point { x, y }
}

impl Point {
    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn from_usizes(x: usize, y: usize) -> Point {
        Point {
            x: x as isize,
            y: y as isize,
        }
    }

    pub const DIRECTIONS_8: &'static [(isize, isize)] = &[
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    #[must_use]
    pub fn down(&self) -> Point {
        point(self.x, self.y.checked_add(1).unwrap())
    }

    #[must_use]
    pub fn left(&self) -> Point {
        point(self.x.checked_sub(1).unwrap(), self.y)
    }

    #[must_use]
    pub fn right(&self) -> Point {
        point(self.x.checked_add(1).unwrap(), self.y)
    }

    #[must_use]
    pub fn up(&self) -> Point {
        point(self.x, self.y.checked_sub(1).unwrap())
    }

    #[must_use]
    pub fn neighbors(&self) -> Vec<Point> {
        vec![self.left(), self.right(), self.up(), self.down()]
    }

    #[must_use]
    pub fn neighbors8(&self) -> Vec<Point> {
        Point::DIRECTIONS_8
            .iter()
            .map(|(dx, dy)| self.delta(*dx, *dy))
            .collect()
    }

    #[must_use]
    pub fn delta(&self, dx: isize, dy: isize) -> Point {
        point(self.x + dx, self.y + dy)
    }

    #[must_use]
    pub fn manhattan_distance(&self, other: &Point) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    /// Return the neighbor in a compass direction.
    #[must_use]
    pub fn step(&self, dir: Dir) -> Point {
        let (dx, dy) = dir.xy_delta();
        self.delta(dx, dy)
    }
}

impl std::str::FromStr for Point {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Point, Self::Err> {
        let (x, y) = s.split_once(',').ok_or("parse point: no comma")?;
        Ok(point(
            x.parse().map_err(|_| "parse x")?,
            y.parse().map_err(|_| "parse y")?,
        ))
    }
}

/// Return the points on a horizontal or vertical line between two points,
/// including those two points.
pub fn line_between(p: Point, q: Point) -> Vec<Point> {
    if p.x == q.x {
        (min(p.y, q.y)..=max(p.y, q.y))
            .map(|y| point(p.x, y))
            .collect()
    } else if p.y == q.y {
        (min(p.x, q.x)..=max(p.x, q.x))
            .map(|x| point(x, p.y))
            .collect()
    } else {
        panic!("points are not in a horizontal or vertical line");
    }
}

/// Compass directions
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, EnumIter)]
pub enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    /// Return the relative x, y for this direction on a map where y runs down.
    pub fn xy_delta(&self) -> (isize, isize) {
        match self {
            Dir::N => (0, -1),
            Dir::S => (0, 1),
            Dir::W => (-1, 0),
            Dir::E => (1, 0),
        }
    }

    /// Return the opposite direction
    pub fn invert(&self) -> Dir {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }

    pub fn turn_right(&self) -> Dir {
        use Dir::*;
        match self {
            N => E,
            E => S,
            S => W,
            W => N,
        }
    }

    pub fn turn_left(&self) -> Dir {
        self.turn_right().invert()
    }
}
