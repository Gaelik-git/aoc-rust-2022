trait Visible {
    fn is_visible(&self, coord: (usize, usize)) -> bool;
    fn compute_visibility(&self, coord: (usize, usize)) -> usize;
}

impl Visible for Vec<Vec<u8>> {
    fn is_visible(&self, coord: (usize, usize)) -> bool {
        let north_three: Vec<u8> = (0..coord.0).map(|s| self[s][coord.1]).collect();
        let south_three: Vec<u8> = (coord.0 + 1..self.len())
            .map(|s| self[s][coord.1])
            .collect();
        let east_three: Vec<u8> = (coord.1 + 1..self.len())
            .map(|s| self[coord.0][s])
            .collect();
        let west_three: Vec<u8> = (0..coord.1).map(|s| self[coord.0][s]).collect();

        //println!("{coord:?}");
        //println!("north : {north_three:?}");
        //println!("south : {south_three:?}");
        //println!("west : {west_three:?}");
        //println!("east : {east_three:?}");

        let curr = self[coord.0][coord.1];
        //println!("curr : {curr:?}");

        north_three.iter().all(|v| v < &curr)
            || south_three.iter().all(|v| v < &curr)
            || east_three.iter().all(|v| v < &curr)
            || west_three.iter().all(|v| v < &curr)
    }

    fn compute_visibility(&self, coord: (usize, usize)) -> usize {
        let north_three: Vec<u8> = (0..coord.0).map(|s| self[s][coord.1]).rev().collect();
        let south_three: Vec<u8> = (coord.0 + 1..self.len())
            .map(|s| self[s][coord.1])
            .collect();
        let east_three: Vec<u8> = (coord.1 + 1..self.len())
            .map(|s| self[coord.0][s])
            .collect();
        let west_three: Vec<u8> = (0..coord.1).map(|s| self[coord.0][s]).rev().collect();

        let curr = self[coord.0][coord.1];
        // println!("{coord:?}");
        // println!("curr : {curr:?}");
        // println!("north : {north_three:?}");
        // println!("south : {south_three:?}");
        // println!("west : {west_three:?}");
        // println!("east : {east_three:?}");

        let val = visibility_check(&north_three, curr) *
          visibility_check(&south_three, curr) *
          visibility_check(&west_three, curr) *
          visibility_check(&east_three, curr);
        // println!("found : {val}");
        val
    }
}

fn visibility_check(trees: &Vec<u8>, curr: u8) -> usize {
  let mut view_ok = true;
    trees
        .iter()
        .take_while(|&x| {
          if !view_ok {
            return false;
          }

          if *x >= curr {
            view_ok = false;
          }
          true
        })
        .collect::<Vec<_>>()
        .len()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::day08::Visible;

    #[test]
    fn adventofcode1() {
        let input = include_str!("input.txt");

        let mut data: Vec<Vec<u8>> = vec![];

        input.lines().for_each(|s| {
            let vec = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
            data.push(vec);
        });

        let mut set: HashSet<(usize, usize)> = HashSet::new();

        let y_size = data.len();
        let x_size = data[0].len();

        for i in 0..x_size {
            set.insert((i, 0));
            set.insert((i, y_size - 1));
        }

        for i in 0..y_size {
            set.insert((0, i));
            set.insert((x_size - 1, i));
        }

        for i in 1..x_size - 1 {
            for j in 1..y_size - 1 {
                if data.is_visible((i, j)) {
                    set.insert((i, j));
                }
            }
        }

        //println!("{:?}", data);
        println!("{:?}", set.len());
    }

    #[test]
    fn adventofcode2() {
        let input = include_str!("input.txt");

        let mut data: Vec<Vec<u8>> = vec![];

        input.lines().for_each(|s| {
            let vec = s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
            data.push(vec);
        });

        let mut max = 0;
        let mut current_coord = (0,0);
        let y_size = data.len();
        let x_size = data[0].len();

        for i in 1..x_size -1  {
            for j in 1..y_size - 1 {
              let calculated = data.compute_visibility((i, j));
              if calculated > max {
                max = calculated;
                current_coord = (i,j);
              }
            }
        }

        //println!("{:?}", data);
        println!("{:?}", max);
        println!("{:?}", current_coord);
    }
}
