use std::{
    collections::HashSet,
    fmt::{Debug, Write},
    ops::{Add, Sub, AddAssign, SubAssign, BitOrAssign},
};
use itertools::Itertools;


#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
  fn delta(&self) -> (i8, i8) {
    match self {
        Direction::UP => (0, -1),
        Direction::DOWN => (0, 1),
        Direction::LEFT => (-1, 0),
        Direction::RIGHT => (1, 0),
    }
  }
}

const KNOTS: usize = 10;

#[derive(Debug)]
struct Grid {
    current_pos: [(isize, isize); KNOTS],
    tail_histo: HashSet<(isize, isize)>,
    moving_index: usize
}

impl Grid {
    pub fn new() -> Self {
        let mut tail_histo = HashSet::new();
        tail_histo.insert((0, 0));

        let mut g = Grid {
            tail_histo,
            current_pos: Default::default(),
            moving_index: 0
        };

        g
    }

    fn moving(&mut self, direction: &Direction) {
  
      
      let delta = direction.delta();
      println!("Moving {direction:?} ({delta:?})");

      let old_head = self.current_pos[self.moving_index];
      let new_head = (old_head.0 + delta.0 as isize, old_head.1 + delta.1 as isize);

      self.current_pos[0] = new_head;

      for i in 1..KNOTS {
        match shoud_follow(self.current_pos[i-1], self.current_pos[i]) {
            Some((dx, dy)) => {
              self.current_pos[i] = (self.current_pos[i].0 + dx, self.current_pos[i].1 + dy);
            },
            None => (),
        }
      }
      
      self.tail_histo.insert(self.current_pos.last().unwrap().clone());

      println!("curr stact : {:?}", self.current_pos);
      //println!("tail_histo : {:?}", self.tail_histo);
      
    }

}



fn shoud_follow(head: (isize, isize), tail: (isize, isize)) -> Option<(isize, isize)> {

  let dist = (head.0 - tail.0, head.1 - tail.1);

  if dist.0.abs() <= 1 && dist.1.abs() <= 1 {
    return None;
  }

  return Some((move_to_do(dist.0), move_to_do(dist.1)));

}

fn move_to_do(i : isize)-> isize {
  if i == 0 {
    return 0;
  }
  return i / i.abs();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
      assert_eq!(None, shoud_follow((0,0), (0,0)));
      assert_eq!(None, shoud_follow((1,0), (0,0)));
      assert_eq!(Some((1,0)), shoud_follow((2,0), (0,0)));
      assert_eq!(Some((1,1)), shoud_follow((2,1), (0,0)));
      assert_eq!(Some((1,-1)), shoud_follow((5, -2), (4,0)));
    }

    #[test]
    fn adventofcode1() -> Result<(), String> {
        let mut grid = Grid::new();
        println!("{grid:?}");

        include_str!("input.txt")
            .lines()
            .map(|f| f.split(" "))
            .map(|mut d| (d.next().unwrap(), d.next().unwrap()))
            .map(|(d, n)| {
              let dist = n.parse::<u8>().unwrap();
              let d = match d {
                "U" => Direction::UP,
                "D" => Direction::DOWN,
                "L" => Direction::LEFT,
                "R" => Direction::RIGHT,
                _ => unreachable!()
              };

              (d, dist)
            }).for_each(|(dir, dist)| {
              for _ in 0..dist {
                grid.moving(&dir);
              }
            });

            //println!("{grid:#?}");
            println!("Result : {}", grid.tail_histo.len());

        Ok(())
    }
}
