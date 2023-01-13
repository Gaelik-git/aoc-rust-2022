use std::{collections::HashSet, fmt::Debug};

use itertools::Itertools;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Coord {
  vals: (i32, i32, i32)
}

impl From<(i32, i32, i32)> for Coord {
    fn from(vals: (i32, i32, i32)) -> Self {
        Coord { vals }
    }
}

impl Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vals.fmt(f)
    }
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
       let vals: (i32, i32, i32) = value.split(",").map(|i| i.parse::<i32>().unwrap()).collect_tuple().unwrap();

       Coord{
        vals
       }
    }
}

impl Coord {
  fn get_voisin(&self) -> [Coord; 6] {
    let vals = self.vals;
    [
      (vals.0 + 1, vals.1, vals.2).into(), (vals.0 - 1, vals.1, vals.2).into(),
      (vals.0, vals.1 + 1, vals.2).into(), (vals.0, vals.1 - 1, vals.2).into(),
      (vals.0, vals.1, vals.2 + 1).into(), (vals.0, vals.1, vals.2 - 1).into()
    ]
  }
}

enum Type {
  Lava,
  Outside,
  Inside
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;


  #[test]
  fn adventofcode1() {
    let lava_set: HashSet<Coord> = include_str!("input.txt").lines().map(|s| s.into()).collect();
    println!("{lava_set:?}");

    let all_possibles: Vec<_> = lava_set.iter().flat_map(|val| val.get_voisin()).collect();
    println!("Max Len : {:?}", all_possibles.iter().unique().collect_vec().len());

    let filtered = all_possibles.iter().filter(|&val| !lava_set.contains(val)).collect_vec();
    println!("Filtered size {}", filtered.len());
    
    let (coord_min_x, coord_max_x) = lava_set.iter().minmax_by_key(|v| v.vals.0).into_option().map(|(min, max)| (min.vals.0, max.vals.0)).unwrap();
    let (coord_min_y, coord_max_y) = lava_set.iter().minmax_by_key(|v| v.vals.1).into_option().map(|(min, max)| (min.vals.1, max.vals.1)).unwrap();
    let (coord_min_z, coord_max_z) = lava_set.iter().minmax_by_key(|v| v.vals.2).into_option().map(|(min, max)| (min.vals.2, max.vals.2)).unwrap();

    let mut air_trapped = HashSet::new();

    for air in filtered {
      //println!("Testing air : {air:?}");
      let mut set = HashSet::new();
      let mut vec: VecDeque<Coord> = VecDeque::new();
      vec.push_back(air.clone());

      let mut res = true;

      while let Some(air) = vec.pop_front() {
        if air.vals.0 < coord_min_x || air.vals.0 > coord_max_x ||
          air.vals.1 < coord_min_y || air.vals.1 > coord_max_y ||
          air.vals.2 < coord_min_z || air.vals.2 > coord_max_z {
            vec = VecDeque::new();
            res = false;
            continue;
          }
        
        if set.contains(&air) {
          continue;
        }

        for next in air.get_voisin() {
          if !lava_set.contains(&next) {
            vec.push_back(next);
          } 
        }

        set.insert(air);
      }

      if res {
        //println!("Is trapped");
        air_trapped.insert(air);
      }else {
        //println!("Is outside");
      }

    }

    println!("air_trapped : {air_trapped:?}");


    let res = all_possibles.iter().filter(|&val| !lava_set.contains(val) && !air_trapped.contains(val)).collect_vec();
    println!("All res : {}", res.len());

  }
}