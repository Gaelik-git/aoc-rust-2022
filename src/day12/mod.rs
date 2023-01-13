use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
pub enum TerrainType {
    Start,
    Target,
    Terrain(usize),
}

impl From<char> for TerrainType {
    fn from(c: char) -> Self {
        match c {
            'S' => TerrainType::Start,
            'E' => TerrainType::Target,
            'a'..='z' => TerrainType::Terrain(c as usize - b'a' as usize),
            _ => unreachable!(),
        }
    }
}

impl TerrainType {
    fn heigh(&self) -> usize {
        match self {
            TerrainType::Start => 0,
            TerrainType::Target => (b'z' - b'a') as usize,
            TerrainType::Terrain(i) => *i,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        GridCoord { x, y }
    }
}

struct CellRecord {
    prev: Option<GridCoord>,
}

pub struct Grid {
    data: Vec<TerrainType>,
    height: usize,
    width: usize,
    visited: HashMap<GridCoord, CellRecord>,
    current: HashSet<GridCoord>,
    num_steps: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        let height = lines.len();
        let width = lines[0].len();

        let data = lines
            .into_iter()
            .flat_map(|s| s.chars())
            .map(|c| TerrainType::from(c))
            .collect();

        Self {
            data,
            height,
            width,
            visited: Default::default(),
            current: Default::default(),
            num_steps: 0,
        }
    }

    fn cell(&self, coord: GridCoord) -> Option<&TerrainType> {
        if !self.valid(coord) {
            return None;
        }

        self.data.get(coord.x + coord.y * self.width)
    }

    fn valid(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn walkable_neighbors(&self, coord: GridCoord) -> impl Iterator<Item = GridCoord> + '_ {
        let curr_ele = self.cell(coord).unwrap().heigh();

        let deltas = [[-1isize, 0], [1, 0], [0, -1], [0, 1]];

        deltas
            .into_iter()
            .filter_map(move |[dx, dy]| {
                Some(GridCoord {
                    x: coord.x.checked_add_signed(dx)?,
                    y: coord.y.checked_add_signed(dy)?,
                })
            })
            .filter(|&coord| self.valid(coord))
            .filter(move |&coord| {
                let new_elev = self.cell(coord).unwrap().heigh();
                new_elev <= curr_ele + 1
            })
    }

    fn step(&mut self) -> bool {
        if self.current.is_empty() {
            // find start coordinate
            let mut start_coord: Option<GridCoord> = None;
            for y in 0..self.height {
                for x in 0..self.width {
                    let coord: GridCoord = (x, y).into();
                    if let TerrainType::Start = self.cell(coord).unwrap() {
                        start_coord = Some(coord);
                        break;
                    }
                }
            }
            let start_coord = start_coord.unwrap();
            self.current.insert(start_coord);
            self.visited.insert(start_coord, CellRecord { prev: None });
            return true;
        }

        let current = std::mem::take(&mut self.current);
        let mut next = HashSet::new();
        let mut visited = std::mem::take(&mut self.visited);

        for curr in current {
            for ncoord in self.walkable_neighbors(curr) {
                if visited.contains_key(&ncoord) {
                    // don't visit it again!
                    continue;
                }
                visited.insert(ncoord, CellRecord { prev: Some(curr) });
                next.insert(ncoord);
            }
        }
        self.current = next;
        self.visited = visited;
        self.num_steps += 1;
        return self.current.len() > 0;
    }


    fn print(&self) {
      print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
      for y in 0..self.height {
        for x in 0..self.width {
          let coords : GridCoord = (x,y).into();
          if self.visited.contains_key(&coords) {
              print!("□");
          }else {
            let d = self.cell((x,y).into()).unwrap();
            match d {
              TerrainType::Start => print!("S"),
              TerrainType::Target => print!("E"),
              TerrainType::Terrain(v) => print!("{}", (b'a' + *v as u8) as char),
            }
          }
        }
        println!("");
      }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, thread::Thread};

    use super::*;

    #[test]
    fn adventofcodeday12_1() {
        let mut explorer = Grid::new(include_str!("input.txt"));

        while explorer.step() {
          explorer.print();
          std::thread::sleep(Duration::from_millis(100));
        }
    }

    #[test]
    fn testing_terrain_type() {
        dbg!(TerrainType::Target.heigh());
        dbg!(TerrainType::from('z').heigh());
    }
}
