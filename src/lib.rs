mod day10;
mod day11;
mod day12;
mod day13;
use aoc_runner_derive::aoc_lib;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

impl std::fmt::Debug for TwoDee<usize> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col = 0;
        let mut row = 0;
        writeln!(f)?;
        for c in self.data.iter() {
            if self.poi == (col, row) {
                write!(f, "X   ")?;
            } else {
                write!(f, "{:>4}", c)?;
            }
            col += 1;
            if col == self.width {
                col = 0;
                row += 1;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for TwoDee<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col = 0;
        let mut row = 0;
        writeln!(f)?;
        for c in self.data.iter() {
            if self.poi == (col, row) {
                write!(f, "X")?;
            } else {
                write!(f, "{c}")?;
            }
            col += 1;
            if col == self.width {
                col = 0;
                row += 1;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for TwoDee<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut col = 0;
        let mut row = 0;
        writeln!(f)?;
        for c in self.data.iter() {
            if self.poi == (col, row) {
                write!(f, "X")?;
            } else {
                write!(f, "{}", if *c { "#" } else { "." })?;
            }
            col += 1;
            if col == self.width {
                col = 0;
                row += 1;
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct TwoDee<T> {
    width: usize,
    data: Vec<T>,
    poi: (usize, usize),
}

impl<R, T> FromIterator<R> for TwoDee<T>
where
    R: Iterator<Item = T>,
{
    fn from_iter<I>(i: I) -> Self
    where
        I: IntoIterator<Item = R>,
    {
        let mut data = Vec::new();
        let mut width = 0;
        for (ix, row) in i.into_iter().enumerate() {
            data.extend(row);
            if width == 0 {
                width = data.len();
            } else {
                assert_eq!(data.len(), width * (ix + 1));
            }
        }
        Self {
            data,
            width,
            poi: (1_000_000, 1_000_000),
        }
    }
}

impl<T> TwoDee<T> {
    #![allow(dead_code)]
    fn new(width: usize) -> Self
    where
        T: Default + Copy,
    {
        let v = T::default();
        Self {
            width,
            data: vec![v; width * width],
            poi: (1_000_000, 1_000_000),
        }
    }

    fn get_two_mut(&mut self, a: Point, b: Point) -> Option<(&mut T, &mut T)> {
        a.guard(self.width)?;
        b.guard(self.width)?;
        let ia = a.x as usize + a.y as usize * self.width;
        let ib = b.x as usize + b.y as usize * self.width;

        match ia.cmp(&ib) {
            std::cmp::Ordering::Less => {
                let (before, after) = self.data.split_at_mut(ib);
                Some((&mut before[ia], &mut after[0]))
            }
            std::cmp::Ordering::Equal => None,
            std::cmp::Ordering::Greater => {
                let (before, after) = self.data.split_at_mut(ia);
                Some((&mut after[0], &mut before[ib]))
            }
        }
    }

    fn map<U>(&self, f: impl FnMut(&T) -> U) -> TwoDee<U> {
        TwoDee {
            width: self.width,
            data: self.data.iter().map(f).collect::<Vec<_>>(),
            poi: self.poi,
        }
    }

    fn get(&self, point: (usize, usize)) -> Option<&T> {
        let (x, y) = point;
        if y >= self.width || x >= self.width {
            return None;
        }
        let i: usize = x + y * self.width;
        self.data.get(i)
    }
    fn get_point(&self, point: Point) -> Option<&T> {
        if point.x < 0 || point.y < 0 {
            None
        } else {
            self.get((point.x as usize, point.y as usize))
        }
    }

    fn get_mut(&mut self, point: (usize, usize)) -> Option<&mut T> {
        let (x, y) = point;
        if y > self.width || x > self.width {
            return None;
        }
        let i: usize = x + y * self.width;
        self.data.get_mut(i)
    }
}

impl<T> std::ops::Index<(usize, usize)> for TwoDee<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for TwoDee<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

impl<T> std::ops::Index<Point> for TwoDee<T> {
    type Output = T;
    fn index(&self, index: Point) -> &Self::Output {
        self.get(index.into()).unwrap()
    }
}

impl<T> std::ops::IndexMut<Point> for TwoDee<T> {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        self.get_mut(index.into()).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const U: Self = Point { x: 0, y: -1 };
    const D: Self = Point { x: 0, y: 1 };
    const L: Self = Point { x: -1, y: 0 };
    const R: Self = Point { x: 1, y: 0 };
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    fn adjacent(self) -> [Point; 4] {
        [
            self + Self::U,
            self + Self::D,
            self + Self::L,
            self + Self::R,
        ]
    }
    fn guard(self, dim: usize) -> Option<Self> {
        if (0..dim).contains(&(self.x as usize)) && (0..dim).contains(&(self.y as usize)) {
            Some(self)
        } else {
            None
        }
    }
    fn u(self) -> Point {
        self + Point::U
    }
    fn d(self) -> Point {
        self + Point::D
    }
    fn l(self) -> Point {
        self + Point::L
    }
    fn r(self) -> Point {
        self + Point::R
    }
}

#[test]
fn point_guard_works() {
    assert_eq!(Some(Point::new(0, 0)), Point::new(0, 0).guard(1));
    assert_eq!(None, Point::new(0, 1).guard(1));
    assert_eq!(None, Point::new(1, 0).guard(1));
    assert_eq!(None, Point::new(1, 1).guard(1));
}

#[test]
fn get_works() {
    let m = [[()].into_iter()].into_iter().collect::<TwoDee<()>>();
    assert_eq!(m.get((0, 0)), Some(&()));
    assert_eq!(m.get((0, 1)), None);
    assert_eq!(m.get((1, 0)), None);
    assert_eq!(m.get((1, 1)), None);
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl std::ops::Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        self * rhs as i32
    }
}

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        let v = value.guard(1_000_000).unwrap();
        (v.x as usize, v.y as usize)
    }
}

aoc_lib! { year = 2024 }
