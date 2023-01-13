use std::{str::FromStr, fmt::Debug, cmp::Ordering::Equal};

use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(untagged)]
enum Element {
  Number(usize),
  List(Vec<Element>)
}

impl Element {
  fn to_slice(&self) -> Vec<&Element> {
    match self {
        Element::List(vec) => vec.iter().collect::<Vec<_>>().clone(),
        e => vec![&e]
    }
  }
}

impl PartialOrd<Element> for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Element::Number(l), Element::Number(r)) => l.partial_cmp(r),
            (l, r) => {
              let l = l.to_slice();
              let r = r.to_slice();
              l.iter()
                .zip(r.iter())
                .map(|(&l,&r)| l.partial_cmp(r))
                .find(|&r| r != Some(Equal))
                .unwrap_or_else(|| l.len().partial_cmp(&r.len()))
            },
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(num) => write!(f, "{}", num),
            Self::List(vec) => f.debug_list().entries(vec).finish()
        }
    }
}


#[derive(Debug)]
struct Signal {
  left: Element,
  right: Element
}

impl Signal {
  fn new(left: &str, right: &str) -> Self {

    let left = serde_json::from_str(left).expect("Left not valid");
    let right = serde_json::from_str(right).expect("Right not valid");

    Self {
      left,
      right
    }

  }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;


  #[test]
  fn adventofcode1() {
    //let sig = Signal::new("[1,1,3,1,1]", "[1,1,5,1,1]");

    let data: Vec<_> = include_str!("./input.txt")
      .split("\n\n")
      .map(|s| s.split("\n"))
      .map(|mut s| Signal::new(s.next().unwrap(), s.next().unwrap()) )
      .collect();

    let mut acc = 0;
    for (i, signal) in data.iter().enumerate() {
      let i = i + 1;

      let comp = signal.left < signal.right;
      println!("Comparing {:?} and {:?} -> left > right : {}", signal.left, signal.right, comp);
      if comp {
        acc += i;
      }
      
    }


    println!("{:?} are in correct order", acc);
  }

  #[test]
  fn adventofcode2() {
    //let sig = Signal::new("[1,1,3,1,1]", "[1,1,5,1,1]");

    let mut data: Vec<Element> = include_str!("./input.txt")
      .lines()
      .filter(|s| !s.is_empty())
      .map(|s| serde_json::from_str(s).unwrap())
      .collect();

    let div1: Element = serde_json::from_str("[[2]]").unwrap();
    let div2: Element = serde_json::from_str("[[6]]").unwrap();

    data.push(div1.clone());
    data.push(div2.clone());

    data.sort();
    
    let div1_index = data.binary_search(&div1).unwrap() + 1;
    let div2_index = data.binary_search(&div2).unwrap() + 1;

    dbg!(div1_index * div2_index);

  }
}