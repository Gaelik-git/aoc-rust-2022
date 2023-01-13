use std::{ops::{Add, Sub, AddAssign, SubAssign}, iter::Sum};

#[derive(Debug)]
struct Cost {
  ore: i32,
  clay: i32,
  obsidian: i32,
  geode: i32
}

impl AddAssign for Cost {
    fn add_assign(&mut self, rhs: Self) {
        self.ore += rhs.ore;
        self.clay += rhs.clay;
        self.obsidian += rhs.obsidian;
        self.geode += rhs.geode;
    }
}

impl SubAssign for Cost {
  fn sub_assign(&mut self, rhs: Self) {
      self.ore -= rhs.ore;
      self.clay -= rhs.clay;
      self.obsidian -= rhs.obsidian;
      self.geode -= rhs.geode;
  }
}

impl Sub for Cost {
  type Output = Cost;

  fn sub(self, rhs: Self) -> Self::Output {
      Cost {
        ore: self.ore - rhs.ore,
        clay: self.clay - rhs.clay,
        obsidian: self.obsidian - rhs.obsidian,
        geode: self.geode - rhs.geode
      }
  }
}

impl Add for Cost {
    type Output = Cost;

    fn add(self, rhs: Self) -> Self::Output {
        Cost {
          ore: self.ore + rhs.ore,
          clay: self.clay + rhs.clay,
          obsidian: self.obsidian + rhs.obsidian,
          geode: self.geode + rhs.geode
        }
    }
}

impl Sum for Cost {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Cost::new(), |a ,b| {
          a + b
        })
    }
}

impl Cost {

  fn new() -> Self {
    Cost {
      ore: 0,
      clay: 0,
      obsidian: 0,
      geode: 0
    }
  }

  fn ore(input: i32) -> Self {
    Cost {
      ore: input,
      clay: 0,
      obsidian: 0,
      geode: 0
    }
  }

  fn clay(input: i32) -> Self {
    Cost {
      ore: 0,
      clay: input,
      obsidian: 0,
      geode: 0
    }
  }

  fn obsidian(input: i32) -> Self {
    Cost {
      ore: 0,
      clay: 0,
      obsidian: input,
      geode: 0
    }
  }


  fn geode(input: i32) -> Self {
    Cost {
      ore: 0,
      clay: 0,
      obsidian: 0,
      geode: input
    }
  }
}

#[derive(Debug)]
struct Blueprint {
    number: i8,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

impl Blueprint {
    fn new(str: &str) -> Blueprint {
        // Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 5 clay. Each geode robot costs 3 ore and 7 obsidian.

        let (_, str) = str.split_once("Blueprint ").unwrap();
        let (number, str) = str.split_once(": Each ore robot costs ").unwrap();
        let (ore_robot_cost, str) = str.split_once(" ore. Each clay robot costs ").unwrap();
        let (clay_robot_cost, str) =
            str.split_once(" ore. Each obsidian robot costs ").unwrap();
        let (obsidian_robot_cost_ore, str) = str.split_once(" ore and ").unwrap();
        let (obsidian_robot_cost_clay, str) = str.split_once(" clay. Each geode robot costs ").unwrap();
        let (geode_robot_cost_ore, str) = str.split_once(" ore and ").unwrap();
        let (geode_robot_cost_obsidian, str) = str.split_once(" obsidian.").unwrap();

        Blueprint {
            number: number.parse::<i8>().unwrap(),
            ore_robot_cost: Cost::ore(ore_robot_cost.parse::<i32>().unwrap()),
            clay_robot_cost: Cost::ore(clay_robot_cost.parse::<i32>().unwrap()),
            obsidian_robot_cost: Cost::ore(obsidian_robot_cost_ore.parse::<i32>().unwrap()) + Cost::clay(obsidian_robot_cost_clay.parse::<i32>().unwrap()),
            geode_robot_cost: Cost::ore(geode_robot_cost_ore.parse::<i32>().unwrap()) + Cost::obsidian(geode_robot_cost_obsidian.parse::<i32>().unwrap()),
        }
    }
}

enum Robots {
  Ore,
  Clay,
  Obsidian,
  Geode
}

impl Robots {
  fn produce(&self) -> Cost {
    match self {
        Robots::Ore => Cost::ore(1),
        Robots::Clay => Cost::clay(1),
        Robots::Obsidian => Cost::obsidian(1),
        Robots::Geode => Cost::geode(1),
    }
  }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn adventofcode1() {
      let bps = include_str!("input.txt").lines().map(|s| Blueprint::new(s)).collect_vec();

      for blueprint in bps {

        let mut robots = vec![Robots::Ore];
        let mut totals = Cost::new();

        for i in 0..24 {
          // generating 
          totals += robots.iter().map(|r| r.produce()).sum();

          //robots.push(Robots::Obsidian);

          println!("{totals:?}");
        }

      }
    }
}
