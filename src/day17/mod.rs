use std::{fmt::Debug, iter::Cycle};

use itertools::Itertools;
use rayon::vec;

enum Form {
    HorizonalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

impl Form {
    fn get_parts(&self) -> Vec<(isize, isize)> {
        match self {
            Form::HorizonalLine => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            Form::Plus => vec![(0, 1), (1, 0), (1, 1), (2, 1), (1, 2)],
            Form::ReverseL => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Form::VerticalLine => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            Form::Square => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        }
    }

    fn get_vertical_heigh(&self) -> isize {
        match self {
            Form::HorizonalLine => 1,
            Form::Plus => 3,
            Form::ReverseL => 3,
            Form::VerticalLine => 4,
            Form::Square => 2,
        }
    }

    fn get_name(&self) -> &str {
        match self {
            Form::HorizonalLine => "HorizonalLine",
            Form::Plus => "Plus",
            Form::ReverseL => "ReverseL",
            Form::VerticalLine => "VerticalLine",
            Form::Square => "Square",
        }
    }

    fn iter() -> [Form; 5] {
        [
            Form::HorizonalLine,
            Form::Plus,
            Form::ReverseL,
            Form::VerticalLine,
            Form::Square,
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    Wall,
    Rock,
}

impl Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Wall => write!(f, "-"),
            Self::Rock => write!(f, "#"),
        }
    }
}

struct Terrain {
    tower: Vec<[Block; 7]>,
    current_block: Option<Vec<(isize, isize)>>,
}

impl Debug for Terrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size = self.tower.len();
        for (y, blocks) in self.tower.iter().enumerate().rev() {
            if y < size - 15 {
                continue;
            }
            write!(f, "|")?;
            for (x, block) in blocks.iter().enumerate() {
                if let Some(vals) = &self.current_block {
                    if vals.contains(&(x as isize, y as isize)) {
                        write!(f, "@")?;
                    } else {
                        block.fmt(f)?;
                    }
                } else {
                    block.fmt(f)?;
                }
            }
            writeln!(f, "|")?;
        }

        Ok(())
    }
}

impl Terrain {
    fn new() -> Self {
        let tower = vec![[Block::Wall; 7]];
        Self {
            tower,
            current_block: None,
        }
    }

    fn starting_pos(&mut self, form: &Form) -> (isize, isize) {
        if self.current_block != None {
            self.current_block = None;
        }

        let x = 2;
        let (index, _) = self
            .tower
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &etage)| etage.contains(&Block::Rock) || etage.contains(&Block::Wall))
            .unwrap();
        let y = index as isize + 4;

        let form_vertical_size = form.get_vertical_heigh();
        while (self.tower.len() as isize) < (y + form_vertical_size) {
            self.tower.push([Block::Empty; 7]);
        }

        let current_block = form
            .get_parts()
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .collect_vec();

        // println!("current_block: {current_block:?}");

        self.current_block = Some(current_block);

        (x, y)
    }

    fn moving(&mut self, dir: &Direction) -> Option<()> {
        self.apply(dir);
        self.apply(&Direction::Down)
    }

    fn apply(&mut self, dir: &Direction) -> Option<()> {
        let mut current_blocks = self.current_block.take().unwrap();
        let old_pos = current_blocks.clone();
        let (dx, dy) = dir.transform();

        for (x, y) in current_blocks.iter_mut() {
            *x += dx;
            *y += dy;
        }
        self.current_block = Some(current_blocks);

        if !self.block_is_ok() {
            match dir {
                Direction::Left | Direction::Right => {
                    self.current_block = Some(old_pos);
                }
                Direction::Down => {
                    for (x, y) in old_pos {
                        self.tower[y as usize][x as usize] = Block::Rock;
                        self.current_block = None;
                    }
                    return None;
                }
            }
        }

        Some(())
    }

    fn block_is_ok(&mut self) -> bool {
        if let Some(vec) = &self.current_block {
            for (x, y) in vec {
                if x >= &7 || x < &0 {
                    return false;
                }

                if y < &0 {
                    return false;
                }

                if self.tower[*y as usize][*x as usize] != Block::Empty {
                    return false;
                }
            }
        }

        return true;
    }

    fn dist_to_high(&self) -> [isize; 7] {
        let mut dist = [isize::MAX; 7];

        let tower_size = self
            .tower
            .iter()
            .filter(|&s| s.contains(&Block::Rock))
            .collect_vec()
            .len();

        for i in (1..=tower_size).rev() {
            for j in 0..7 {
                if dist[j] != isize::MAX {
                    continue;
                }

                if self.tower[i][j] != Block::Empty {
                    dist[j] = (tower_size - i) as isize;
                }
            }

            if dist.iter().all(|s| s != &isize::MAX) {
                return dist;
            }
        }

        dist.iter_mut()
            .filter(|s| s == &&isize::MAX)
            .for_each(|s| *s = tower_size as isize);

        dist
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    fn transform(&self) -> (isize, isize) {
        match self {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, -1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, process::exit};

    use itertools::Itertools;

    use super::*;

    #[test]
    fn adventofcode1() {
        let dirs: Vec<Direction> = include_str!("input.txt")
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.into())
            .collect_vec();
        let forms = Form::iter();
        //println!("{directions:?}");

        let mut terrain = Terrain::new();

        let mut dir_index = None;
        let mut dir = &dirs[0];

        let mut hashmap: HashMap<(usize, usize, [isize; 7]), (u128, usize)> = Default::default();
        let mut short_cut_found = false;

        let max_nbr_block = 1000000000000;

        let mut skipped_height = 0;

        let mut i = 0u128;
        while i < max_nbr_block {
            let form_index = (i % 5) as usize;
            let form = &forms[form_index];
            // print!(
            //     "Placing {} ({form_index}; {i}) dir index : {} {dir_index:?}",
            //     form.get_name(),
            //     dir_index.unwrap_or(0) % dirs.len()
            // );
            terrain.starting_pos(form);

            if i == 1989 {
                println!("{terrain:?}");
            }

            loop {
                let dist_to_highest = terrain.dist_to_high();
                dir_index = Some(dir_index.take().map(|v| v + 1).unwrap_or(0));

                let key = (form_index, dir_index.unwrap() % dirs.len(), dist_to_highest);
                let mut current_height = terrain
                    .tower
                    .iter()
                    .filter(|l| l.contains(&Block::Rock))
                    .collect_vec()
                    .len();

                dir = &dirs[dir_index.unwrap() % dirs.len()];
                let res = terrain.moving(dir);

                if res.is_none() {
                    //println!(" ---- Placed after {dir_index:?}");

                    if !short_cut_found && hashmap.contains_key(&key) {
                      short_cut_found = true;
                      let (number_of_block, height) = hashmap.get(&key).unwrap();
                      let cycle_size = dirs.len();
                      let height_effect = current_height - height;
                      let block_effect = i - number_of_block;
                      //println!("");
                      //println!("Found a loop : {cycle_size}, {height_effect} higher with {block_effect} blocks");
                      //println!("{key:?}");
  
                      let nbr_loop = (max_nbr_block - i) / block_effect;
  
                      i += nbr_loop * block_effect;
                      skipped_height += nbr_loop * height_effect as u128;
                      //dir_index = Some(dir_index.take().map(|v| v+5).unwrap_or(0));
  
                      //i -= 1;
                      //println!("After using the loop {nbr_loop} times : {i} blocks used - {skipped_height} skipped_height");
                      break;
                  } else if !short_cut_found {
                      hashmap.insert(
                          (form_index, dir_index.unwrap() % dirs.len(), dist_to_highest),
                          (i, current_height),
                      );
                  }

                    break;
                }
            }

            if i == 1989 {
                //println!("{terrain:?}");
            }

            i += 1;
        }

        println!(
            "res: {}",
            terrain
                .tower
                .iter()
                .filter(|l| l.contains(&Block::Rock))
                .collect_vec()
                .len() as u128
                + skipped_height
        );
    }
}
