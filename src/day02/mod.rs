#[cfg(test)]
mod tests {
    use std::{io::{BufRead, BufReader}, fs::File, cmp::Reverse};
    use itertools::Itertools;


  #[test]
  fn adventofcode1() {
    let t: u64 = include_str!("input.txt")
      .lines()
      .map(|s| {
        let mut it = s.split(' ');
        return (it.next().unwrap(), it.next().unwrap())
      })
      .map(|g| points1(g))
      .sum();

    println!("{:?}", t);
  }

  fn points1(game: (&str, &str)) -> u64{
    match game {
      ("A", "X") => 1 + 3,
      ("B", "Y") => 2 + 3,
      ("C", "Z") => 3 + 3,
      ("A", "Y") => 2 + 6,
      ("B", "Z") => 3 + 6,
      ("C", "X") => 1 + 6,
      ("A", "Z") => 3 + 0,
      ("B", "X") => 1 + 0,
      ("C", "Y") => 2 + 0,
      _ => unreachable!()
    }
  }

  #[test]
  fn adventofcode2() {
    let t: u64 = include_str!("input.txt")
      .lines()
      .map(|s| {
        let mut it = s.split(' ');
        return (it.next().unwrap(), it.next().unwrap())
      })
      .map(|g| points2(g))
      .sum();

    println!("{:?}", t);
  }

  fn points2(game: (&str, &str)) -> u64{
    match game {
      ("A", "X") => 3 + 0,
      ("B", "X") => 1 + 0,
      ("C", "X") => 2 + 0,

      ("A", "Y") => 1 + 3,
      ("B", "Y") => 2 + 3,
      ("C", "Y") => 3 + 3,

      ("A", "Z") => 2 + 6,
      ("B", "Z") => 3 + 6,
      ("C", "Z") => 1 + 6,
      _ => unreachable!()
    }
  }

}