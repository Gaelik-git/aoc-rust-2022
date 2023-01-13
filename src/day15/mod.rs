use std::{fmt::Debug, collections::btree_map::Range, ops::RangeInclusive};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn dist(&self, other: &Point) -> usize {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);

        x_dist + y_dist
    }
}

impl Into<Point> for (usize, usize) {
    fn into(self) -> Point {
        Point {
            x: self.0 as isize,
            y: self.1 as isize,
        }
    }
}

impl Into<Point> for &str {
    fn into(self) -> Point {
        let mut splits = self.split(", ");
        let x = &splits.next().unwrap()["x=".len()..];
        let y = &splits.next().unwrap()["y=".len()..];

        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Point,
    beacon: Beacon,
    radius: usize,
}

impl Sensor {
  fn y_range(&self) -> RangeInclusive<isize> {
    self.pos.y - self.radius as isize..=self.pos.y + self.radius as isize
  }
}

#[derive(Debug)]
struct Beacon {
    pos: Point,
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Type {
    Sensor,
    Beacon,
    Unknown,
    Impossible,
}

impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sensor => write!(f, "S"),
            Self::Beacon => write!(f, "B"),
            Self::Unknown => write!(f, "."),
            Self::Impossible => write!(f, "#"),
        }
    }
}

trait ParseLine {
    fn read(&self) -> Sensor;
}

impl ParseLine for &str {
    fn read(&self) -> Sensor {
        let mut splits = self.split(":");

        let sensor_pos: Point = splits.next().unwrap()["Sensor at ".len()..].into();
        let beacon_pos: Point = splits.next().unwrap()[" closest beacon is at ".len()..].into();

        let radius = sensor_pos.dist(&beacon_pos);

        Sensor {
            pos: sensor_pos,
            beacon: Beacon { pos: beacon_pos },
            radius: radius,
        }
    }
}

struct State {
    size: usize,
    state: Vec<Type>,
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.size {
            for i in 0..self.size {
                write!(f, "{:?}", self.cell(&(i, j).into()).unwrap())?;
            }
            writeln!(f, "")?;
        }
        write!(f, "")
    }
}

impl State {
    fn new(size: usize) -> Self {
        Self {
            size,
            state: vec![Type::Unknown; size * size],
        }
    }

    fn cell(&self, p: &Point) -> Option<&Type> {
        if p.x < 0 || p.x > self.size as isize || p.y < 0 || p.y > self.size as isize {
            return None;
        }

        let x = p.x as usize;
        let y = p.y as usize;
        self.state.get(x + y * self.size)
    }

    fn cell_mut(&mut self, p: &Point) -> Option<&mut Type> {
        if p.x < 0 || p.x > self.size as isize || p.y < 0 || p.y > self.size as isize {
            return None;
        }

        let x = p.x as usize;
        let y = p.y as usize;
        self.state.get_mut(x + y * self.size)
    }
}

struct Ranges{
  ranges: Vec<RangeInclusive<isize>>
}

impl Ranges {
  fn new(y_max: isize) -> Self {
    Self {
      ranges: vec![0..=y_max]
    }
  }

  fn minus(&mut self, minus: &RangeInclusive<isize>) {

    self.ranges = self.ranges.iter().flat_map(|left| Ranges::range_ops(left, minus)).collect();

  }

  fn range_ops(left: &RangeInclusive<isize>, right: &RangeInclusive<isize>) -> Vec<RangeInclusive<isize>> {
    if left.start() > right.end() || left.end() < right.start() {
      return vec![left.clone()];
    }

    if right.contains(left.start()) && right.contains(left.end()) {
      return vec![];
    }

    if left.start() < right.start() && left.end() > right.end() {
      return vec![*left.start()..=*right.start()-1, *right.end()+1..=*left.end()];
    }
    
    if left.contains(&(right.end() + 1)) {
      return vec![*right.end()+1..=*left.end()];
    }

    if left.contains(&(right.start() - 1)) {
      return vec![*left.start()..=*right.start()-1];
    }

    println!("doing {:?} - {:?}", left, right);
    unreachable!()
  }
 }

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    // #[test]
    // fn adventofcode1() {
    //     let data: Vec<_> = include_str!("input.txt")
    //         .lines()
    //         .map(|s| s.read())
    //         .collect();

    //     let mut all_points: HashSet<Point> = HashSet::new();
    //     let mut sensor_beacon = HashSet::new();

    //     let target_y = 2000000;

    //     for (s, b) in data.iter() {
    //       let dist = s.0.dist(&b.0);
    //       //println!("{s:?} - {b:?} : {dist}");

    //       sensor_beacon.insert(s.0.clone());
    //       sensor_beacon.insert(b.0.clone());

    //       let closest = Point {
    //         x: s.0.x,
    //         y: target_y
    //       };

    //       let dist_to_y_target = closest.dist(&s.0);

    //       if dist_to_y_target > dist {
    //         continue;
    //       }

    //       let move_left = (dist - dist_to_y_target) as isize;
    //       //println!("{s:?} -> {move_left}");

    //       for x in s.0.x-move_left..=s.0.x+move_left {
    //         let point = Point {
    //             x,
    //             y: target_y
    //           };

    //         if !sensor_beacon.contains(&point) {
    //           all_points.insert(
    //             point
    //           );
    //         }
    //       }
    //     }
    //     dbg!(all_points.len());
    // }

    #[test]
    fn adventofcode2() {
        let sensors: Vec<_> = include_str!("input.txt")
            .lines()
            .map(|s| s.read())
            .collect();

        println!("Data parsed");

        let max_y = 4_000_000;

        for j in 0..=max_y {
            let mut ranges = Ranges::new(max_y);
            //println!("For line {j}");
            for sensor in sensors.iter() {
              if !sensor.y_range().contains(&j) {
                continue;
              }

              //println!("Sensor in range {:?} (r: {})", sensor.pos, sensor.radius);
              let dist_to_j = (sensor.pos.y - j ).abs();
              let left_distance =  sensor.radius as isize - dist_to_j;
              let range_to_sub = (sensor.pos.x-left_distance)..=(sensor.pos.x+left_distance);

              ranges.minus(&range_to_sub);

              //println!("ranges : {:?}", ranges.ranges);
            }

            if !ranges.ranges.is_empty() {
              println!("{:?}", ranges.ranges);
              assert!(ranges.ranges.len() == 1);
              assert!(ranges.ranges[0].start() == ranges.ranges[0].end());
              let x = ranges.ranges[0].start();
              let y = j;
              println!("It's here ? : x:{x} y:{y} -> {}", x * 4_000_000 + y);
              return;
            }
        }

    }


    #[test]
    fn ops_test() {

      let left = 0..=3;
      let right = 1..=3;
      let res = Ranges::range_ops(&left,&right);

      assert_eq!(res[0], 0..=0);
    }
}
