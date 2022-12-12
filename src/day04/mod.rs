

mod pairs {
    use std::ops::{Range, RangeInclusive};

  #[derive(Debug)]
  pub struct Pair(Elf, Elf);

  impl Pair {

    pub fn overlap(&self) -> bool {
      self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    pub fn partial_overlap(&self) -> bool {
      self.0.partial_overlap(&self.1) || self.1.partial_overlap(&self.0)
    }
  }

  impl From<&str> for Pair {
    fn from(input: &str) -> Self {
        let mut vals = input.split(",");
        let elf1 = vals.next().unwrap();
        let elf2 = vals.next().unwrap();
        Pair(elf1.into(), elf2.into())
    }
}

  #[derive(Debug)]
  pub struct Elf(RangeInclusive<usize>);

  impl Elf {
    pub fn contains(&self, other: &Elf) -> bool {
      self.0.contains(other.0.start()) && self.0.contains(other.0.end())
    }

    pub fn partial_overlap(&self, other: &Elf) -> bool {
      self.0.contains(other.0.start()) || self.0.contains(other.0.end())
    }
  }

  impl From<&str> for Elf {
    fn from(input: &str) -> Self {
        let mut vals = input.split("-");
        let min = vals.next().unwrap().parse::<usize>().unwrap();
        let max = vals.next().unwrap().parse::<usize>().unwrap();
        Elf(min..=max)
    }
}
}


#[cfg(test)]
mod tests {

  use std::ops::RangeInclusive;

use super::pairs::Pair;

  #[test]
  fn adventofcode1() {
    let pairs : Vec<Pair> = include_str!("input.txt").lines()
    .map(|f| f.into())
    .filter(|p: &Pair| p.overlap())
    .collect();

    println!("{:?}", pairs.len());
  }

  #[test]
  fn adventofcode2() {

    let pairs : Vec<Pair> = include_str!("input.txt").lines()
    .map(|f| f.into())
    .filter(|p: &Pair| p.partial_overlap())
    .collect();

    println!("{:?}", pairs.len());
    
  }
}