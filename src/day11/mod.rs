use std::{fmt::Debug, str::{Lines, FromStr}};


#[derive(Clone)]
enum Operation {
  Mult(usize),
  Add(usize),
  Square
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let mut s = s.split(" ");

      match (s.next().unwrap(), s.next().unwrap(), s.next().unwrap()) {
        ("old", "*", "old") => Ok(Operation::Square),
        ("old", "*", v) => Ok(Operation::Mult(v.parse::<usize>().unwrap())),
        ("old", "+", v) => Ok(Operation::Add(v.parse::<usize>().unwrap())),
        _ => unreachable!()
      }


    }
}

struct Monkey {
    items: Vec<usize>,
    operand: Operation,
    test: usize,
    target_true: usize,
    target_false: usize,
    nbr_inspected: usize
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("target_true", &self.target_true)
            .field("target_false", &self.target_false)
            .field("nbr_inspected", &self.nbr_inspected)
            .finish()
    }
}

impl Monkey {
    fn new(lines: &mut Lines) -> Self {
        let items: Vec<_> = lines.next().unwrap().trim()["Starting items: ".len()..].split(",").map(|f| f.trim()).filter_map(|e| e.parse::<usize>().ok()).collect();
        let operand = Operation::from_str(&lines.next().unwrap().trim()["Operation: new = ".len()..]).unwrap();
        let test = lines.next().unwrap().trim()["Test: divisible by ".len()..]
            .parse::<usize>()
            .unwrap();
        let target_true = lines.next().unwrap().trim()["If true: throw to monkey ".len()..]
            .parse::<usize>()
            .unwrap();
        let target_false = lines.next().unwrap().trim()["If false: throw to monkey ".len()..]
            .parse::<usize>()
            .unwrap();

        lines.next();


        Monkey {
          target_true,
          target_false,
          test,
          items,
          operand,
          nbr_inspected: 0
        }
    }

    fn test(&self, to_check: usize) -> bool {
      to_check % self.test == 0
    }

    fn operate(&self, input: usize) -> usize {
      match self.operand {
        Operation::Mult(x) => x * input,
        Operation::Add(x) => x + input,
        Operation::Square => input * input,
      }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn adventofcode1() {
    //     let mut lines = include_str!("input.txt").lines();
    //     let mut monkeys: Vec<Monkey> = vec![];
    //     while let Some(_) = lines.next() {
    //       monkeys.push(Monkey::new(&mut lines));
    //     }

    //     for i in 0..20 {
    //       for m in 0..monkeys.len() {
    //         let mut to_move: Vec<(usize, usize)> = vec![];
    //         for worry in monkeys[m].items.iter() {
    //           let new_worry = (monkeys[m].operation)(*worry);
    //           let new_worry = new_worry / 3;
    //           let test_result = (monkeys[m].test)(new_worry);
    //           if test_result {
    //             let target = monkeys[m].target_true;
    //             to_move.push((target, new_worry));
    //           }else {
    //             let target = monkeys[m].target_false;
    //             to_move.push((target, new_worry));
    //           }
    //         }

    //         monkeys[m].nbr_inspected += monkeys[m].items.len();
    //         monkeys[m].items = vec![];
    //         for (target, val) in to_move {
    //           monkeys[target].items.push(val);
    //         }
    //       }
    //     }

    //     dbg!(monkeys);
    // }

    #[test]
    fn adventofcode2() {
        let mut lines = include_str!("input.txt").lines();
        let mut monkeys: Vec<Monkey> = vec![];
        while let Some(_) = lines.next() {
          monkeys.push(Monkey::new(&mut lines));
        }

        let modulo: usize = monkeys.iter().map(|m| m.test).product();

        for i in 0..10000 {
          for m in 0..monkeys.len() {
            let mut to_move: Vec<(usize, usize)> = vec![];
            for worry in monkeys[m].items.iter() {
              
              let new_worry = monkeys[m].operate(*worry);
              let new_worry = new_worry % modulo;
              //println!("{i} For monkey ({m}) : base {worry:?} -> {new_worry}");

              let test_result = monkeys[m].test(new_worry);
              if test_result {
                let target = monkeys[m].target_true;
                to_move.push((target, new_worry));
              }else {
                let target = monkeys[m].target_false;
                to_move.push((target, new_worry));
              }
            }

            monkeys[m].nbr_inspected += monkeys[m].items.len();
            monkeys[m].items = vec![];
            for (target, val) in to_move {
              monkeys[target].items.push(val);
            }
          }
        }

        dbg!(monkeys);
    }

}
