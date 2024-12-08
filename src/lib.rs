use aoc_runner_derive::aoc_lib;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

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
            poi: (0, 0),
        }
    }
}

impl<T> TwoDee<T> {
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
    fn get(&self, point: (usize, usize)) -> Option<&T> {
        let (x, y) = point;
        if y > self.width || x > self.width {
            return None;
        }
        let i: usize = x + y * self.width;
        self.data.get(i)
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn guard(self, dim: usize) -> Option<Self> {
        if self.x < 0 || self.y < 0 || self.x as usize >= dim || self.y as usize >= dim {
            None
        } else {
            Some(self)
        }
    }
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

impl From<Point> for (usize, usize) {
    fn from(value: Point) -> Self {
        let v = value.guard(1_000_000).unwrap();
        (v.x as usize, v.y as usize)
    }
}

aoc_lib! { year = 2024 }
