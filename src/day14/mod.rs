use std::{collections::HashSet, fmt::Debug, str::FromStr};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn path_to(&self, other: &Coord) -> Vec<Coord> {
        let mut res = vec![];

        if self.y == other.y {
            for x in self.x.min(other.x)..self.x.max(other.x) + 1 {
                res.push(Coord { x, y: self.y });
            }
        }

        if self.x == other.x {
            for y in self.y.min(other.y)..self.y.max(other.y) + 1 {
                res.push(Coord { y, x: self.x });
            }
        }

        res
    }

    fn get_next(&self) -> [Coord; 3] {
        [
            (self.x, self.y + 1).into(),
            (self.x - 1, self.y + 1).into(),
            (self.x + 1, self.y + 1).into(),
        ]
    }
}

impl FromStr for Coord {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coord: Vec<usize> = s
            .split(",")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();

        assert!(coord.len() == 2);

        Ok(Coord {
            x: coord[0],
            y: coord[1],
        })
    }
}

impl From<(usize, usize)> for Coord {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug)]
struct RockPath {
    coords: Vec<Coord>,
}

impl RockPath {
    fn all_coords(&self) -> HashSet<Coord> {
        self.coords
            .windows(2)
            .flat_map(|win| win[0].path_to(&win[1]))
            .collect()
    }
}

impl FromIterator<Coord> for RockPath {
    fn from_iter<T: IntoIterator<Item = Coord>>(iter: T) -> Self {
        let coords: Vec<_> = iter.into_iter().collect();
        coords.into()
    }
}

impl From<Vec<Coord>> for RockPath {
    fn from(coords: Vec<Coord>) -> Self {
        RockPath { coords }
    }
}

#[derive(Eq, PartialEq)]
enum Element {
    Sand,
    Rock,
    Air,
    Source,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sand => write!(f, "o"),
            Self::Rock => write!(f, "#"),
            Self::Air => write!(f, "."),
            Self::Source => write!(f, "+"),
        }
    }
}

struct Grid {
    data: Vec<Element>,
    x_min: usize,
    x_max: usize,
    y_max: usize,
    curr: Option<Coord>,
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.heigh() {
            for e in self.data[y * self.width()..y * self.width() + self.width()].iter() {
                write!(f, "{:?} ", e)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

impl Grid {
    fn new(input: HashSet<Coord>, x_min: usize, x_max: usize, y_max: usize) -> Self {
        let mut data = vec![];

        for y in 0..=y_max {
            for x in x_min..=x_max {
                if x == 500 && y == 0 {
                    data.push(Element::Source);
                } else if input.contains(&(x, y).into()) {
                    data.push(Element::Rock);
                } else {
                    data.push(Element::Air);
                }
            }
        }

        Grid {
            x_min,
            x_max,
            y_max,
            data,
            curr: Default::default(),
        }
    }

    fn cell(&self, coord: &Coord) -> Result<&Element, ()> {
        if coord.x < self.x_min || coord.x > self.x_max || coord.y > self.y_max {
            return Err(());
        }

        return Ok(&self.data[(coord.x - self.x_min) + coord.y * self.width()]);
    }

    fn cell_mut(&mut self, coord: &Coord) -> Result<&mut Element, ()> {
        if coord.x < self.x_min || coord.x > self.x_max || coord.y > self.y_max {
            return Err(());
        }

        let width = self.width();
        return Ok(&mut self.data[(coord.x - self.x_min) + coord.y * width]);
    }

    fn width(&self) -> usize {
        1 + self.x_max - self.x_min
    }

    fn heigh(&self) -> usize {
        self.y_max + 1
    }

    fn step(&mut self) -> Result<Status, ()> {
        if self.curr == None {
            self.curr = Some((500, 0).into());
        }

        let curr = self.curr.take().unwrap();

        for possible_next in curr.get_next() {
            let val = self.cell(&possible_next);

            match val {
                Err(_) => {
                  *self.cell_mut(&curr).unwrap() = Element::Sand;
                  self.curr = None;
          
                  return Ok(Status::Resting);
                },
                Ok(val) => match val {
                    Element::Air => {
                        self.curr = Some(possible_next);
                        return Ok(Status::Falling);
                    }
                    Element::Sand => (),
                    Element::Rock => (),
                    Element::Source => unreachable!(),
                },
            }
        }

        //can go anywhere -> become at rest
        *self.cell_mut(&curr).unwrap() = Element::Sand;

        if curr == (500, 0).into() {
          return Err(());
        }

        self.curr = None;
        Ok(Status::Resting)
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Status {
    Falling,
    Resting,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn adventofcode1() {
        let mut data: HashSet<Coord> = include_str!("input.txt")
            .lines()
            .map(|s| {
                s.split(" -> ")
                    .map(|s| Coord::from_str(s).unwrap())
                    .collect()
            })
            .flat_map(|rp: RockPath| rp.all_coords())
            .collect();

        let min_x =  0; // data.iter().map(|c| c.x).min().unwrap();
        let max_x =  data.iter().map(|c| c.x).max().unwrap() * 2;

        let max_y = data.iter().map(|c| c.y).max().unwrap() + 2;

        let bottom: RockPath = vec![(min_x, max_y).into(), (max_x, max_y).into()].into();
        data.extend(bottom.all_coords().into_iter());

        let mut grid = Grid::new(data, min_x, max_x, max_y);

        //println!("{:?}", grid);
        while let Ok(val) = grid.step() {
            //println!("{:?}", val);
            if val == Status::Resting {
                //println!("{:?}", grid);
            }
        }
        println!("{:?}", grid);

        let nbr_sand = grid
            .data
            .iter()
            .filter(|&e| e == &Element::Sand)
            .collect::<Vec<_>>()
            .len();
        dbg!(nbr_sand);

        dbg!(min_x, max_x, max_y);
    }
}
